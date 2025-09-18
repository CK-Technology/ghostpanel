# Azure Active Directory SSO Setup

> **Configure Microsoft Azure AD authentication for GhostPanel**
> Enterprise-grade authentication for your Bolt container management platform

---

## 🎯 Overview

Azure Active Directory (Azure AD) integration allows your organization to use existing Microsoft accounts for GhostPanel authentication. This includes:

- **Corporate accounts** - Employees with Azure AD accounts
- **Guest accounts** - External users invited to your tenant
- **Multi-factor authentication** - Enhanced security with MFA
- **Conditional access** - Advanced security policies

---

## 📋 Prerequisites

- Azure AD tenant (comes with Office 365, Azure subscription, or free tier)
- Global Administrator or Application Administrator permissions
- GhostPanel instance with HTTPS enabled (required for production)

---

## 🚀 Step 1: Register Application in Azure

### 1.1 Open Azure Portal
1. Go to [Azure Portal](https://portal.azure.com)
2. Sign in with your Azure AD administrator account
3. Navigate to **Azure Active Directory**

### 1.2 Create App Registration
1. Click **App registrations** in the left sidebar
2. Click **New registration**
3. Fill in the application details:

```
Name: GhostPanel Container Management
Supported account types: Accounts in this organizational directory only
Redirect URI:
  Platform: Web
  URL: https://your-ghostpanel.domain.com/auth/callback
```

4. Click **Register**

### 1.3 Note Important Values
After registration, copy these values (you'll need them later):

```bash
Application (client) ID: 12345678-1234-1234-1234-123456789012
Directory (tenant) ID: 87654321-4321-4321-4321-210987654321
```

---

## 🔑 Step 2: Configure Authentication

### 2.1 Set Redirect URIs
1. In your app registration, go to **Authentication**
2. Under **Redirect URIs**, add all environments:

```
Production:  https://ghostpanel.yourdomain.com/auth/callback
Staging:     https://staging-ghostpanel.yourdomain.com/auth/callback
Development: http://localhost:9443/auth/callback  (for testing only)
```

### 2.2 Configure Platform Settings
1. Under **Platform configurations**, click **Web**
2. Enable these options:
   - ✅ **Access tokens** (used for implicit grant flow)
   - ✅ **ID tokens** (used for OpenID Connect)

### 2.3 Set Logout URL
```
Front-channel logout URL: https://your-ghostpanel.domain.com/auth/logout
```

---

## 🔐 Step 3: Create Client Secret

### 3.1 Generate Secret
1. Go to **Certificates & secrets**
2. Click **New client secret**
3. Configure:
   - **Description**: GhostPanel Production Secret
   - **Expires**: 24 months (recommended)
4. Click **Add**

### 3.2 Copy Secret Value
⚠️ **IMPORTANT**: Copy the secret value immediately - it won't be shown again!

```bash
Client Secret: xYz123AbC456DeF789GhI012JkL345MnO678PqR
```

---

## 🎯 Step 4: Configure API Permissions

### 4.1 Add Required Permissions
1. Go to **API permissions**
2. Click **Add a permission**
3. Select **Microsoft Graph**
4. Choose **Delegated permissions**
5. Add these permissions:

```
✅ openid              - Basic OpenID Connect
✅ profile             - User's profile information
✅ email               - User's email address
✅ User.Read           - Read user's profile
```

### 4.2 Grant Admin Consent (Optional)
For organization-wide deployment:
1. Click **Grant admin consent for [Your Organization]**
2. Click **Yes** to confirm
3. Permissions should show "Granted for [Your Organization]"

---

## 🔧 Step 5: Configure GhostPanel

### 5.1 Environment Variables
Set these environment variables in your GhostPanel deployment:

```bash
# Enable Azure AD authentication
GPANEL_AZURE_ENABLED=true

# Azure AD configuration
GPANEL_AZURE_CLIENT_ID=12345678-1234-1234-1234-123456789012
GPANEL_AZURE_CLIENT_SECRET=xYz123AbC456DeF789GhI012JkL345MnO678PqR
GPANEL_AZURE_TENANT_ID=87654321-4321-4321-4321-210987654321

# Optional: Restrict to specific domain
GPANEL_AZURE_DOMAIN_HINT=yourcompany.com
```

### 5.2 Boltfile Configuration
```toml
# Boltfile.toml
[services.gpanel-web.auth.oidc.azure]
enabled = true
client_id = "${GPANEL_AZURE_CLIENT_ID}"
client_secret = "${GPANEL_AZURE_CLIENT_SECRET}"
tenant_id = "${GPANEL_AZURE_TENANT_ID}"
domain_hint = "yourcompany.com"  # Optional
```

### 5.3 Docker Compose Example
```yaml
version: '3.8'
services:
  gpanel-web:
    image: ghostpanel/web:latest
    ports:
      - "9443:9443"
    environment:
      - GPANEL_AUTH_ENABLED=true
      - GPANEL_AZURE_ENABLED=true
      - GPANEL_AZURE_CLIENT_ID=${AZURE_CLIENT_ID}
      - GPANEL_AZURE_CLIENT_SECRET=${AZURE_CLIENT_SECRET}
      - GPANEL_AZURE_TENANT_ID=${AZURE_TENANT_ID}
      - GPANEL_REDIRECT_URL=https://ghostpanel.yourdomain.com/auth/callback
```

---

## 🎮 Step 6: Configure User Roles (Optional)

### 6.1 Create App Roles
1. In your app registration, go to **App roles**
2. Click **Create app role**
3. Create roles for different access levels:

**GhostPanel Admin Role:**
```json
{
  "allowedMemberTypes": ["User"],
  "description": "Full access to GhostPanel management",
  "displayName": "GhostPanel Admin",
  "id": "11111111-1111-1111-1111-111111111111",
  "isEnabled": true,
  "value": "ghostpanel.admin"
}
```

**GhostPanel User Role:**
```json
{
  "allowedMemberTypes": ["User"],
  "description": "Read-only access to containers",
  "displayName": "GhostPanel User",
  "id": "22222222-2222-2222-2222-222222222222",
  "isEnabled": true,
  "value": "ghostpanel.user"
}
```

**Gaming Manager Role:**
```json
{
  "allowedMemberTypes": ["User"],
  "description": "Manage gaming containers and GPU resources",
  "displayName": "Gaming Manager",
  "id": "33333333-3333-3333-3333-333333333333",
  "isEnabled": true,
  "value": "ghostpanel.gaming"
}
```

### 6.2 Assign Users to Roles
1. Go to **Azure AD** → **Enterprise applications**
2. Find your GhostPanel app
3. Go to **Users and groups**
4. Click **Add user/group**
5. Select users and assign appropriate roles

---

## 🧪 Step 7: Test Authentication

### 7.1 Test Login Flow
1. Navigate to your GhostPanel instance: `https://your-ghostpanel.domain.com`
2. Click **Sign in with Azure AD**
3. You should be redirected to Microsoft login
4. After successful login, you should be back in GhostPanel

### 7.2 Verify JWT Token
```bash
# Check authentication status
curl -H "Authorization: Bearer your-jwt-token" \
     https://your-ghostpanel.domain.com/api/auth/me

# Expected response:
{
  "user": {
    "id": "user@yourcompany.com",
    "name": "John Doe",
    "email": "user@yourcompany.com",
    "roles": ["ghostpanel.admin"],
    "provider": "azure"
  }
}
```

---

## 🔍 Troubleshooting

### Common Issues

**❌ "AADSTS50011: The redirect URI specified in the request does not match"**
```bash
# Solution: Check redirect URI matches exactly
App Registration → Authentication → Redirect URIs
Configured: https://ghostpanel.domain.com/auth/callback
GhostPanel:  GPANEL_REDIRECT_URL=https://ghostpanel.domain.com/auth/callback
```

**❌ "AADSTS700016: Application not found in the directory"**
```bash
# Solution: Check tenant ID and client ID
GPANEL_AZURE_TENANT_ID=correct-tenant-id
GPANEL_AZURE_CLIENT_ID=correct-client-id
```

**❌ "Invalid client secret"**
```bash
# Solution: Generate new client secret
# Client secrets expire - check expiration date in Azure
```

**❌ "Insufficient privileges to complete the operation"**
```bash
# Solution: Grant admin consent for API permissions
Azure AD → App registrations → Your App → API permissions → Grant admin consent
```

### Debug Logging
```bash
# Enable debug logging for Azure AD
RUST_LOG=debug,azure_auth=trace
GPANEL_LOG_LEVEL=debug

# Check logs for authentication details
docker logs ghostpanel-web | grep -i azure
```

---

## 🏢 Advanced Configuration

### Multi-Tenant Support
```bash
# Allow users from any Azure AD tenant
GPANEL_AZURE_TENANT_ID=common

# Allow users from any Microsoft account (including personal)
GPANEL_AZURE_TENANT_ID=consumers

# Allow both Azure AD and personal Microsoft accounts
GPANEL_AZURE_TENANT_ID=organizations
```

### Conditional Access Integration
- Configure conditional access policies in Azure AD
- Require MFA for GhostPanel access
- Restrict access based on location, device, or risk level

### Custom Claims
```toml
[services.gpanel-web.auth.oidc.azure]
# Request additional claims
additional_scopes = ["Groups.Read.All"]
group_claims = true
department_claims = true
```

---

## 📊 Monitoring

### Azure AD Sign-in Logs
1. Go to **Azure AD** → **Monitoring** → **Sign-ins**
2. Filter by your GhostPanel application
3. Monitor successful and failed authentication attempts

### GhostPanel Analytics
```bash
# Check authentication metrics
curl https://your-ghostpanel.domain.com/api/metrics | grep azure_auth

# View active Azure AD sessions
curl -H "Authorization: Bearer admin-jwt" \
     https://your-ghostpanel.domain.com/api/admin/sessions
```

---

**🔒 Your GhostPanel is now secured with Azure Active Directory!** Users can sign in with their corporate Microsoft accounts and access container management based on their assigned roles.

🚀 **Next Steps:**
- Configure user roles and permissions
- Set up conditional access policies
- Monitor authentication metrics
- Train users on the new login process