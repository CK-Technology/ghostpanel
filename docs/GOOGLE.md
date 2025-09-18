# Google OAuth2 Setup for GhostPanel

This guide walks through setting up Google OAuth2 integration for GhostPanel authentication.

## Prerequisites

- Google account with access to Google Cloud Console
- GhostPanel instance with HTTPS enabled (required for OAuth2)
- Admin access to your GhostPanel deployment

## Step 1: Create Google Cloud Project

1. Go to [Google Cloud Console](https://console.cloud.google.com/)
2. Click **Select a project** → **New Project**
3. Enter project details:
   - **Project name**: `GhostPanel-Auth` (or your preferred name)
   - **Organization**: Select your organization (if applicable)
4. Click **Create**

## Step 2: Enable Google+ API

1. In the Google Cloud Console, go to **APIs & Services** → **Library**
2. Search for "Google+ API"
3. Click on **Google+ API** and click **Enable**
4. Alternatively, enable **Google Identity** service for newer implementations

## Step 3: Configure OAuth Consent Screen

1. Go to **APIs & Services** → **OAuth consent screen**
2. Select **External** user type (or **Internal** if using Google Workspace)
3. Click **Create**
4. Fill in the required information:

### App Information
```
App name: GhostPanel
User support email: your-email@domain.com
Developer contact information: your-email@domain.com
```

### App Domain (Optional but Recommended)
```
Application home page: https://your-ghostpanel-domain.com
Application privacy policy link: https://your-ghostpanel-domain.com/privacy
Application terms of service link: https://your-ghostpanel-domain.com/terms
```

### Authorized Domains
```
your-ghostpanel-domain.com
```

5. Click **Save and Continue**

### Scopes
1. Click **Add or Remove Scopes**
2. Add these scopes:
   - `../auth/userinfo.email`
   - `../auth/userinfo.profile`
   - `openid`
3. Click **Update** → **Save and Continue**

### Test Users (if using External)
1. Add test user emails who can access during development
2. Click **Save and Continue**

## Step 4: Create OAuth2 Credentials

1. Go to **APIs & Services** → **Credentials**
2. Click **Create Credentials** → **OAuth client ID**
3. Select **Web application**
4. Configure the client:

```
Name: GhostPanel Web Client
Authorized JavaScript origins:
  - https://your-ghostpanel-domain.com:9443
  - https://localhost:9443 (for development)

Authorized redirect URIs:
  - https://your-ghostpanel-domain.com:9443/auth/google/callback
  - https://localhost:9443/auth/google/callback (for development)
```

5. Click **Create**
6. **Important**: Copy and securely store:
   - **Client ID** (starts with numbers, ends with `.apps.googleusercontent.com`)
   - **Client Secret** (random string)

## Step 5: Configure GhostPanel

### Environment Variables

Add these environment variables to your GhostPanel deployment:

```bash
# Google OAuth2 Configuration
GOOGLE_CLIENT_ID="your-client-id.apps.googleusercontent.com"
GOOGLE_CLIENT_SECRET="your-client-secret"
GOOGLE_REDIRECT_URI="https://your-ghostpanel-domain.com:9443/auth/google/callback"

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
    "GOOGLE_CLIENT_ID=${GOOGLE_CLIENT_ID}",
    "GOOGLE_CLIENT_SECRET=${GOOGLE_CLIENT_SECRET}",
    "GOOGLE_REDIRECT_URI=https://${DOMAIN}:9443/auth/google/callback",
    "OAUTH_ENABLED=true",
    "JWT_SECRET=${JWT_SECRET}"
]
```

## Step 6: Test Authentication

1. Start your GhostPanel instance
2. Navigate to `https://your-ghostpanel-domain.com:9443`
3. Click **Sign in with Google**
4. You should be redirected to Google's OAuth consent screen
5. Grant permissions and verify you're redirected back to GhostPanel

## Security Best Practices

### Client Secret Protection
- Never expose client secrets in frontend code
- Use environment variables for secrets
- Rotate client secrets regularly
- Consider using Google Secret Manager for production

### Redirect URI Validation
```bash
# Ensure only your domains are authorized
Authorized redirect URIs should only include:
- Your production domain
- Localhost for development (remove in production)
```

### HTTPS Requirements
- Google OAuth2 requires HTTPS in production
- Use valid SSL certificates
- Consider using Let's Encrypt for automatic certificate management

## Troubleshooting

### Common Issues

#### "redirect_uri_mismatch" Error
```
Error: The redirect URI in the request does not match the ones authorized for the OAuth client.
```
**Solution**: Ensure the redirect URI in your GhostPanel configuration exactly matches what's configured in Google Cloud Console.

#### "invalid_client" Error
```
Error: The OAuth client was not found.
```
**Solution**: Verify your client ID is correct and the OAuth client hasn't been deleted.

#### "access_denied" Error
```
Error: The user or Google denied the request.
```
**Solution**: Check OAuth consent screen configuration and ensure user has access permissions.

### Development vs Production

#### Development Setup
```bash
# Use localhost for development
GOOGLE_REDIRECT_URI="https://localhost:9443/auth/google/callback"

# Authorized JavaScript origins
https://localhost:9443
```

#### Production Setup
```bash
# Use your domain for production
GOOGLE_REDIRECT_URI="https://your-domain.com:9443/auth/google/callback"

# Authorized JavaScript origins
https://your-domain.com:9443
```

### Logging and Debugging

Enable debug logging in GhostPanel:

```bash
RUST_LOG="debug"
GHOSTPANEL_AUTH_DEBUG="true"
```

Check logs for OAuth flow:
```bash
bolt logs ghostpanel-web | grep -i "google\|oauth\|auth"
```

## API Reference

### Google OAuth2 Endpoints Used

```
Authorization URL: https://accounts.google.com/o/oauth2/v2/auth
Token URL: https://oauth2.googleapis.com/token
User Info URL: https://www.googleapis.com/oauth2/v2/userinfo
```

### Scopes Requested

```
openid: OpenID Connect authentication
email: User's email address
profile: Basic profile information (name, picture)
```

### User Information Retrieved

```json
{
  "id": "google-user-id",
  "email": "user@gmail.com",
  "verified_email": true,
  "name": "User Name",
  "given_name": "User",
  "family_name": "Name",
  "picture": "https://profile-picture-url",
  "locale": "en"
}
```

## Additional Resources

- [Google OAuth2 Documentation](https://developers.google.com/identity/protocols/oauth2)
- [Google Cloud Console](https://console.cloud.google.com/)
- [OAuth2 Security Best Practices](https://tools.ietf.org/html/draft-ietf-oauth-security-topics)
- [GhostPanel Authentication Architecture](./SSO.md)

## Support

For issues specific to Google OAuth2 integration:
1. Check Google Cloud Console logs
2. Verify OAuth client configuration
3. Test with Google's OAuth2 Playground
4. Review GhostPanel authentication logs

For general GhostPanel authentication issues, see [SSO.md](./SSO.md).