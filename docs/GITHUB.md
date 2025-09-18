# GitHub OAuth Setup for GhostPanel

This guide walks through setting up GitHub OAuth integration for GhostPanel authentication.

## Prerequisites

- GitHub account (personal or organization)
- GhostPanel instance with HTTPS enabled (required for OAuth)
- Admin access to your GhostPanel deployment

## Step 1: Create GitHub OAuth App

### For Personal Account

1. Go to [GitHub Settings](https://github.com/settings/profile)
2. Click **Developer settings** in the left sidebar
3. Click **OAuth Apps**
4. Click **New OAuth App**

### For Organization

1. Go to your organization's page on GitHub
2. Click **Settings** tab
3. Click **Developer settings** in the left sidebar
4. Click **OAuth Apps**
5. Click **New OAuth App**

## Step 2: Configure OAuth Application

Fill in the application details:

### Basic Information
```
Application name: GhostPanel
Homepage URL: https://your-ghostpanel-domain.com
Application description: Container management platform for Bolt containers
```

### OAuth Configuration
```
Authorization callback URL: https://your-ghostpanel-domain.com:9443/auth/github/callback
```

**Important**: The callback URL must exactly match your GhostPanel configuration.

### Development Setup (Optional)
For development, you may want to create a separate OAuth app:

```
Application name: GhostPanel (Development)
Homepage URL: https://localhost:9443
Authorization callback URL: https://localhost:9443/auth/github/callback
```

## Step 3: Generate Client Credentials

1. Click **Register application**
2. You'll be redirected to your app's settings page
3. Note down the **Client ID** (publicly visible)
4. Click **Generate a new client secret**
5. **Important**: Copy and securely store the **Client Secret** immediately
   - This secret will only be shown once
   - Store it in a secure password manager or environment variable

## Step 4: Configure Application Permissions

### Email Access (Required)
GitHub OAuth provides public profile information by default. To access email addresses:

1. In your OAuth app settings, ensure these scopes are requested:
   - `user:email` - Access user email addresses
   - `read:user` - Read user profile information

**Note**: These scopes are configured in your GhostPanel application, not in GitHub's interface.

### Organization Access (Optional)
If you want to restrict access to organization members:

1. Enable **Organization access restrictions** in your app settings
2. Configure which organizations can use your app
3. Request `read:org` scope in your application

## Step 5: Configure GhostPanel

### Environment Variables

Add these environment variables to your GhostPanel deployment:

```bash
# GitHub OAuth Configuration
GITHUB_CLIENT_ID="your-github-client-id"
GITHUB_CLIENT_SECRET="your-github-client-secret"
GITHUB_REDIRECT_URI="https://your-ghostpanel-domain.com:9443/auth/github/callback"

# General OAuth Settings
OAUTH_ENABLED="true"
JWT_SECRET="your-secure-jwt-secret-key"
```

### Boltfile Configuration

Update your `Boltfile.toml`:

```toml
[services.ghostpanel-web]
# ... existing configuration ...
environment = [
    "GITHUB_CLIENT_ID=${GITHUB_CLIENT_ID}",
    "GITHUB_CLIENT_SECRET=${GITHUB_CLIENT_SECRET}",
    "GITHUB_REDIRECT_URI=https://${DOMAIN}:9443/auth/github/callback",
    "OAUTH_ENABLED=true",
    "JWT_SECRET=${JWT_SECRET}"
]
```

### Docker Compose Alternative

If using Docker Compose:

```yaml
services:
  ghostpanel-web:
    environment:
      - GITHUB_CLIENT_ID=${GITHUB_CLIENT_ID}
      - GITHUB_CLIENT_SECRET=${GITHUB_CLIENT_SECRET}
      - GITHUB_REDIRECT_URI=https://${DOMAIN}:9443/auth/github/callback
      - OAUTH_ENABLED=true
      - JWT_SECRET=${JWT_SECRET}
```

## Step 6: Test Authentication

1. Start your GhostPanel instance
2. Navigate to `https://your-ghostpanel-domain.com:9443`
3. Click **Sign in with GitHub**
4. You should be redirected to GitHub's authorization page
5. Click **Authorize [your-app-name]**
6. Verify you're redirected back to GhostPanel with successful login

## Advanced Configuration

### Organization-Only Access

To restrict access to specific GitHub organizations, configure in your application:

```rust
// In your GhostPanel auth configuration
pub struct GitHubConfig {
    pub client_id: String,
    pub client_secret: String,
    pub allowed_organizations: Vec<String>, // Optional: ["my-org", "another-org"]
    pub require_org_membership: bool,
}
```

Environment variables:
```bash
GITHUB_ALLOWED_ORGS="my-org,another-org"
GITHUB_REQUIRE_ORG_MEMBERSHIP="true"
```

### Team-Based Access Control

For more granular access control:

```bash
GITHUB_ALLOWED_TEAMS="my-org/admin-team,my-org/dev-team"
GITHUB_ADMIN_TEAMS="my-org/admin-team"
```

### Private Email Handling

GitHub users can make their email addresses private. Handle this in your configuration:

```bash
# Fallback to GitHub username if email is private
GITHUB_ALLOW_PRIVATE_EMAIL="true"
GITHUB_EMAIL_FALLBACK="username"
```

## Security Best Practices

### Client Secret Protection
- Never expose client secrets in frontend code
- Use environment variables for secrets
- Rotate client secrets regularly
- Use GitHub's secret scanning to detect exposed secrets

### Webhook Security (Optional)
If you plan to use GitHub webhooks:

```bash
GITHUB_WEBHOOK_SECRET="your-webhook-secret"
```

### Rate Limiting
GitHub has API rate limits. Configure appropriate caching:

```bash
GITHUB_API_CACHE_TTL="3600"  # Cache user info for 1 hour
GITHUB_ORG_CACHE_TTL="7200"  # Cache org membership for 2 hours
```

## Troubleshooting

### Common Issues

#### "redirect_uri_mismatch" Error
```
Error: The redirect_uri parameter value is not allowed for this application.
```
**Solution**: Ensure the redirect URI in your GhostPanel configuration exactly matches what's configured in your GitHub OAuth app.

#### "bad_verification_code" Error
```
Error: The code passed is incorrect or expired.
```
**Solution**: This usually indicates a timing issue or the authorization code was already used. Ensure your system clock is correct.

#### "incorrect_client_credentials" Error
```
Error: The client_id and/or client_secret passed are incorrect.
```
**Solution**: Verify your client ID and secret are correct and properly set in environment variables.

### Organization Access Issues

#### User Not in Required Organization
```
Error: User is not a member of required organization.
```
**Solution**:
1. Verify the user is a member of the required organization
2. Check organization membership visibility settings
3. Ensure the `read:org` scope is requested

#### Private Organization Membership
If organization membership is private:

```bash
# Request public org membership only, or ensure users make membership public
GITHUB_REQUIRE_PUBLIC_ORG_MEMBERSHIP="true"
```

### Development vs Production

#### Development Setup
```bash
# Use localhost for development
GITHUB_REDIRECT_URI="https://localhost:9443/auth/github/callback"

# Separate OAuth app recommended for development
GITHUB_CLIENT_ID="dev-client-id"
GITHUB_CLIENT_SECRET="dev-client-secret"
```

#### Production Setup
```bash
# Use your domain for production
GITHUB_REDIRECT_URI="https://your-domain.com:9443/auth/github/callback"

# Production OAuth app
GITHUB_CLIENT_ID="prod-client-id"
GITHUB_CLIENT_SECRET="prod-client-secret"
```

### Logging and Debugging

Enable debug logging:

```bash
RUST_LOG="debug"
GHOSTPANEL_AUTH_DEBUG="true"
GITHUB_DEBUG="true"
```

Check authentication logs:
```bash
bolt logs ghostpanel-web | grep -i "github\|oauth\|auth"
```

## API Reference

### GitHub OAuth2 Endpoints Used

```
Authorization URL: https://github.com/login/oauth/authorize
Token URL: https://github.com/login/oauth/access_token
User API URL: https://api.github.com/user
User Email URL: https://api.github.com/user/emails
Organizations URL: https://api.github.com/user/orgs
```

### Scopes Requested

```
user:email - Access to user email addresses
read:user - Read access to user profile information
read:org - Read access to organization membership (if org restrictions enabled)
```

### User Information Retrieved

```json
{
  "login": "username",
  "id": 12345678,
  "email": "user@example.com",
  "name": "User Name",
  "avatar_url": "https://avatars.githubusercontent.com/u/12345678",
  "company": "Company Name",
  "location": "Location",
  "bio": "User bio",
  "html_url": "https://github.com/username"
}
```

### Organization Information (if requested)

```json
[
  {
    "login": "my-org",
    "id": 87654321,
    "url": "https://api.github.com/orgs/my-org",
    "description": "My Organization"
  }
]
```

## Webhooks Integration (Advanced)

For real-time organization membership updates:

### Create Webhook in GitHub

1. Go to your organization settings
2. Click **Webhooks**
3. Click **Add webhook**
4. Configure:
   ```
   Payload URL: https://your-domain.com:9443/webhooks/github
   Content type: application/json
   Secret: your-webhook-secret
   Events: Member, Organization
   ```

### GhostPanel Webhook Configuration

```bash
GITHUB_WEBHOOK_ENABLED="true"
GITHUB_WEBHOOK_SECRET="your-webhook-secret"
GITHUB_WEBHOOK_PATH="/webhooks/github"
```

## Additional Resources

- [GitHub OAuth Documentation](https://docs.github.com/en/developers/apps/building-oauth-apps)
- [GitHub API Documentation](https://docs.github.com/en/rest)
- [OAuth Scopes](https://docs.github.com/en/developers/apps/building-oauth-apps/scopes-for-oauth-apps)
- [GitHub OAuth Best Practices](https://docs.github.com/en/developers/apps/building-oauth-apps/authorizing-oauth-apps)
- [GhostPanel Authentication Architecture](./SSO.md)

## Support

For issues specific to GitHub OAuth integration:
1. Check GitHub OAuth app settings
2. Verify redirect URI configuration
3. Test with GitHub's OAuth flow manually
4. Review GhostPanel authentication logs
5. Check GitHub API rate limits

For general GhostPanel authentication issues, see [SSO.md](./SSO.md).