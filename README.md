# casdoor-rust-sdk

[![GitHub last commit](https://img.shields.io/github/last-commit/casdoor/casdoor-rust-sdk)](https://github.com/casdoor/casdoor-rust-sdk/commits/master)
[![Crates.io](https://img.shields.io/crates/v/casdoor-rust-sdk.svg)](https://crates.io/crates/casdoor-rust-sdk)
[![Docs](https://docs.rs/casdoor-rust-sdk/badge.svg)](https://docs.rs/casdoor-rust-sdk)
[![CI](https://github.com/casdoor/casdoor-rust-sdk/workflows/CI/badge.svg)](https://github.com/casdoor/casdoor-rust-sdk/actions)
[![Discord](https://img.shields.io/discord/1022748306096537660?logo=discord&label=discord&color=5865F2)](https://discord.gg/5rPsrAzK7S)

This is Casdoor's SDK for Rust, which will allow you to easily connect your application to the Casdoor authentication system without having to implement it from scratch.

Casdoor SDK is very simple to use. We will show you the steps below.

```toml
[dependencies]
casdoor-rust-sdk = <latest-version>
```

## Step1. Init SDK

Initialization requires 6 parameters, which are all string type:

| Name (in order) | Must | Description                                         |
| --------------- | ---- | --------------------------------------------------- |
| endpoint        | Yes  | Casdoor Server Url, such as `http://localhost:8000` |
| client_id       | Yes  | Client ID for the Casdoor application               |
| client_secret   | Yes  | Client secret for the Casdoor application           |
| certificate     | Yes  | x509 certificate content of Application.cert        |
| org_name        | Yes  | The name for the Casdoor organization               |
| app_name        | No   | The name for the Casdoor application                |

```rust
// init from params.
let app = CasdoorConfig::new(endpoint, client_id, client_secret, certificate, org_name);

// init from toml file, file_path should be absolute path. (recommend)
let conf = CasdoorConfig::from_toml(file_path).unwrap().as_str()).unwrap();
```

## Step2. Get service and use

Now provide the following services:

| Service | Description |
|---------|-------------|
| `AuthService` | Authentication service |
| `UserService` | User management service |
| `OrganizationService` | Organization management service |
| `ApplicationService` | Application management service |
| `GroupService` | Group management service |
| `RoleService` | Role management service |
| `PermissionService` | Permission management service |
| `ResourceService` | Resource management service |
| `CertService` | Certificate management service |

You can create them like:

```rust
let auth_service = AuthService::new(&conf);
let user_service = UserService::new(&conf);
let organization_service = OrganizationService::new(&conf);
let application_service = ApplicationService::new(&conf);
let group_service = GroupService::new(&conf);
let role_service = RoleService::new(&conf);
let permission_service = PermissionService::new(&conf);
let resource_service = ResourceService::new(&conf);
let cert_service = CertService::new(&conf);
```

## Step3. Interact with the sdk services

The SDK supports basic operations for all services.

### Auth Service

- `get_auth_token(code)`, get the auth token.
- `parse_jwt_token(token)`, parse jwt token.
- `get_signin_url(redirect_url)`, get the sign-in URL.
- `get_signup_url(redirect_url)`, get the sign-up URL.
- `get_signup_url_enable_password()`, get the sign-up URL with password enabled.
- `get_user_profile_url(uname, token)`, get the user profile URL.
- `get_my_profile_url(token)`, get the current user's profile URL.

### User Service

- `get_user(name)`, get one user by user name.
- `get_users()`, get all users.
- `get_sorted_users(sorter, limit)`, get sorted users with limit.
- `get_user_count(is_online)`, get user count.
- `get_user_with_email(name, email)`, get user by name and email.
- `add_user(User)`, add a new user.
- `update_user(User)`, update user info.
- `delete_user(User)`, delete a user.

### Organization Service

- `get_organizations()`, get all organizations.
- `get_organization(name)`, get one organization by name.
- `get_default_application()`, get the default application.
- `get_organization_names()`, get all organization names.
- `add_organization(organization)`, add a new organization.
- `update_organization(organization)`, update organization info.
- `delete_organization(organization)`, delete an organization.

### Application Service

- `get_applications()`, get all applications.
- `get_application(name)`, get one application by name.
- `get_user_application()`, get the current user's application.
- `get_organization_applications()`, get all applications for an organization.
- `add_application(application)`, add a new application.
- `update_application(application)`, update application info.
- `delete_application(application)`, delete an application.

### Group Service

- `get_groups()`, get all groups.
- `get_group(name)`, get one group by name.
- `add_group(group)`, add a new group.
- `update_group(group)`, update group info.
- `delete_group(group)`, delete a group.
- `upload_groups(groups)`, upload multiple groups.

### Role Service

- `get_roles()`, get all roles.
- `get_role(name)`, get one role by name.
- `add_role(role)`, add a new role.
- `update_role(role)`, update role info.
- `delete_role(role)`, delete a role.
- `upload_roles(roles)`, upload multiple roles.

### Permission Service

- `get_permissions()`, get all permissions.
- `get_permissions_by_submitter(submitter)`, get permissions by submitter.
- `get_permissions_by_role(role)`, get permissions by role.
- `get_permission(name)`, get one permission by name.
- `add_permission(permission)`, add a new permission.
- `update_permission(permission)`, update permission info.
- `delete_permission(permission)`, delete a permission.
- `upload_permissions(permissions)`, upload multiple permissions.

### Resource Service

- `get_resources()`, get all resources.
- `get_resource(name)`, get one resource by name.
- `add_resource(resource)`, add a new resource.
- `update_resource(resource)`, update resource info.
- `delete_resource(resource)`, delete a resource.
- `upload_resource(resource)`, upload a resource.

### Cert Service

- `get_certs()`, get all certificates.
- `get_global_certs()`, get all global certificates.
- `get_cert(name)`, get one certificate by name.
- `add_cert(cert)`, add a new certificate.
- `update_cert(cert)`, update certificate info.
- `delete_cert(cert)`, delete a certificate.
