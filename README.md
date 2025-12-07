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
| `InvitationService` | Invitation management service |
| `ProviderService` | Provider management service |
| `ModelService` | Model management service |
| `AdapterService` | Adapter management service |
| `EnforcerService` | Enforcer management service |
| `SessionService` | Session management service |
| `TokenService` | Token management service |
| `SystemService` | System management service |
| `LdapService` | LDAP integration service |
| `RecordService` | Record management service |
| `FormService` | Form management service |
| `SyncerService` | Syncer management service |
| `WebhookService` | Webhook management service |
| `CasService` | CAS (Central Authentication Service) service |
| `ScimService` | SCIM (System for Cross-domain Identity Management) service |
| `ProductService` | Product management service |
| `OrderService` | Order management service |
| `PaymentService` | Payment management service |

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
let invitation_service = InvitationService::new(&conf);
let provider_service = ProviderService::new(&conf);
let model_service = ModelService::new(&conf);
let adapter_service = AdapterService::new(&conf);
let enforcer_service = EnforcerService::new(&conf);
let session_service = SessionService::new(&conf);
let token_service = TokenService::new(&conf);
let system_service = SystemService::new(&conf);
let ldap_service = LdapService::new(&conf);
let record_service = RecordService::new(&conf);
let form_service = FormService::new(&conf);
let syncer_service = SyncerService::new(&conf);
let webhook_service = WebhookService::new(&conf);
let cas_service = CasService::new(&conf);
let scim_service = ScimService::new(&conf);
let product_service = ProductService::new(&conf);
let order_service = OrderService::new(&conf);
let payment_service = PaymentService::new(&conf);
```

## Step3. Interact with the sdk services

The SDK supports basic operations for all services.

### Auth Service

- `get_auth_token(code)`, get the auth token.
- `login(username, password)`, login with username and password.
- `signup(username, password, email, phone)`, signup a new user.
- `logout(token)`, logout with token.
- `sso_logout(token)`, SSO logout with token.
- `get_account(token)`, get account info with token.
- `get_userinfo(token)`, get user info with token.
- `parse_jwt_token(token)`, parse jwt token.
- `get_signin_url(redirect_url)`, get the sign-in URL.
- `get_signup_url(redirect_url)`, get the sign-up URL.
- `get_signup_url_enable_password()`, get the sign-up URL with password enabled.
- `get_user_profile_url(uname, token)`, get the user profile URL.
- `get_my_profile_url(token)`, get the current user's profile URL.
- `get_app_login(app_name)`, get application login info.
- `get_dashboard()`, get dashboard info.
- `unlink(provider, token)`, unlink a provider from user account.
- `get_saml_login(app_name)`, get SAML login info.
- `get_saml_metadata()`, get SAML metadata.
- `get_qrcode(content)`, get QR code.
- `get_captcha_status()`, get captcha status.
- `get_oauth_token(code, grant_type, redirect_uri)`, get OAuth token.
- `refresh_token(refresh_token, grant_type)`, refresh token.
- `introspect_token(token, token_type_hint)`, introspect token.
- `handle_callback(code, state)`, handle OAuth callback.
- `device_auth(device_code, user_code)`, device authentication.

#### WebAuthn Related Methods

- `webauthn_signup_begin(user, display_name)`, begin WebAuthn signup process.
- `webauthn_signup_finish(response)`, finish WebAuthn signup process.
- `webauthn_signin_begin(user)`, begin WebAuthn signin process.
- `webauthn_signin_finish(response)`, finish WebAuthn signin process.

#### MFA Related Methods

- `mfa_setup_initiate(user, type)`, initiate MFA setup.
- `mfa_setup_verify(user, type, code)`, verify MFA setup.
- `mfa_setup_enable(user, type, enabled)`, enable/disable MFA.
- `delete_mfa(user, type)`, delete MFA.
- `set_preferred_mfa(user, type)`, set preferred MFA type.

#### Well-known Endpoints

- `get_openid_configuration()`, get OpenID configuration.
- `get_jwks()`, get JWKS (JSON Web Key Set).

#### FaceID Related Methods

- `faceid_register(user, image)`, register FaceID for user.
- `faceid_verify(user, image)`, verify FaceID for user.
- `faceid_delete(user)`, delete FaceID for user.

### User Service

- `get_user(name)`, get one user by user name.
- `get_users()`, get all users.
- `get_global_users()`, get all global users.
- `get_sorted_users(sorter, limit)`, get sorted users with limit.
- `get_user_count(is_online)`, get user count.
- `get_user_with_email(name, email)`, get user by name and email.
- `add_user(User)`, add a new user.
- `add_user_keys(user, public_key, signature)`, add user keys.
- `update_user(User)`, update user info.
- `delete_user(User)`, delete a user.
- `upload_users(users)`, upload multiple users.
- `remove_user_from_group(user, group)`, remove user from group.
- `set_password(user, password)`, set user password.
- `check_user_password(user, password)`, check if password is correct.
- `send_verification_code(type, target)`, send verification code.
- `verify_code(type, target, code)`, verify code.
- `get_captcha()`, get captcha.
- `verify_captcha(ticket, rand_str)`, verify captcha.
- `get_email_and_phone(user)`, get user's email and phone.
- `reset_email_or_phone(user, type, value, code)`, reset user's email or phone.
- `get_verifications()`, get all verifications.

### Invitation Service

- `get_invitations()`, get all invitations.
- `get_invitation(name)`, get one invitation by name.
- `get_invitation_info(code)`, get invitation info by code.
- `add_invitation(invitation)`, add a new invitation.
- `update_invitation(invitation)`, update invitation info.
- `delete_invitation(invitation)`, delete an invitation.
- `verify_invitation(code)`, verify invitation by code.
- `send_invitation(invitation)`, send invitation.

### Provider Service

- `get_providers()`, get all providers.
- `get_provider(name)`, get one provider by name.
- `get_global_providers()`, get all global providers.
- `add_provider(provider)`, add a new provider.
- `update_provider(provider)`, update provider info.
- `delete_provider(provider)`, delete a provider.

### Model Service

- `get_models()`, get all models.
- `get_model(name)`, get one model by name.
- `add_model(model)`, add a new model.
- `update_model(model)`, update model info.
- `delete_model(model)`, delete a model.

### Adapter Service

- `get_adapters()`, get all adapters.
- `get_adapter(name)`, get one adapter by name.
- `add_adapter(adapter)`, add a new adapter.
- `update_adapter(adapter)`, update adapter info.
- `delete_adapter(adapter)`, delete an adapter.
- `get_policies()`, get all policies.
- `get_filtered_policies(field_index, field_values)`, get filtered policies.
- `add_policy(params)`, add a policy.
- `update_policy(old_params, new_params)`, update a policy.
- `remove_policy(params)`, remove a policy.

### Enforcer Service

- `get_enforcers()`, get all enforcers.
- `get_enforcer(name)`, get one enforcer by name.
- `add_enforcer(enforcer)`, add a new enforcer.
- `update_enforcer(enforcer)`, update enforcer info.
- `delete_enforcer(enforcer)`, delete an enforcer.
- `enforce(params)`, enforce a policy.
- `batch_enforce(params_list)`, batch enforce policies.
- `get_all_objects()`, get all objects.
- `get_all_actions()`, get all actions.
- `get_all_roles()`, get all roles.
- `run_casbin_command(command)`, run Casbin command.
- `refresh_engines()`, refresh engines.

### Session Service

- `get_sessions()`, get all sessions.
- `get_session(name)`, get one session by name.
- `add_session(session)`, add a new session.
- `update_session(session)`, update session info.
- `delete_session(session)`, delete a session.
- `is_session_duplicated(user)`, check if session is duplicated.

### Token Service

- `get_tokens()`, get all tokens.
- `get_token(name)`, get one token by name.
- `add_token(token)`, add a new token.
- `update_token(token)`, update token info.
- `delete_token(token)`, delete a token.

### System Service

- `get_system_info()`, get system information.
- `get_version_info()`, get version information.
- `health()`, check system health.
- `get_prometheus_info()`, get Prometheus information.
- `get_metrics()`, get system metrics.
- `send_email(title, content, sender, receivers)`, send email.
- `send_sms(phone, content)`, send SMS.
- `send_notification(title, content, receivers)`, send notification.

### Ldap Service

- `get_ldap_users(ldap_id)`, get users from LDAP.
- `get_ldaps()`, get all LDAP configurations.
- `get_ldap(name)`, get one LDAP configuration by name.
- `add_ldap(ldap)`, add a new LDAP configuration.
- `update_ldap(ldap)`, update LDAP configuration.
- `delete_ldap(ldap)`, delete LDAP configuration.
- `sync_ldap_users(ldap_id)`, sync users from LDAP.

### Record Service

- `get_records()`, get all records.
- `get_records_by_filter(filter)`, get records by filter.
- `add_record(record)`, add a new record.

### Form Service

- `get_global_forms()`, get all global forms.
- `get_forms()`, get all forms.
- `get_form(name)`, get one form by name.
- `add_form(form)`, add a new form.
- `update_form(form)`, update form.
- `delete_form(form)`, delete form.

### Syncer Service

- `get_syncers()`, get all syncers.
- `get_syncer(name)`, get one syncer by name.
- `add_syncer(syncer)`, add a new syncer.
- `update_syncer(syncer)`, update syncer.
- `delete_syncer(syncer)`, delete syncer.
- `run_syncer(name)`, run syncer.
- `test_syncer_db(syncer)`, test syncer database connection.

### Webhook Service

- `get_webhooks()`, get all webhooks.
- `get_webhook(name)`, get one webhook by name.
- `add_webhook(webhook)`, add a new webhook.
- `update_webhook(webhook)`, update webhook.
- `delete_webhook(webhook)`, delete webhook.
- `get_webhook_event_type()`, get webhook event types.

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

### Cas Service

- `cas_validate(ticket, service)`, CAS 1.0: Validate ticket.
- `cas_service_validate(ticket, service, pgt_url)`, CAS 2.0: Service ticket validation.
- `cas_proxy_validate(ticket, service, pgt_url)`, CAS 2.0: Proxy ticket validation.
- `cas3_service_validate(ticket, service, pgt_url)`, CAS 3.0: Service ticket validation with attributes.
- `cas3_proxy_validate(ticket, service, pgt_url)`, CAS 3.0: Proxy ticket validation with attributes.
- `cas_proxy(pgt, target_service)`, CAS: Proxy.
- `cas_get_user_profile(ticket, service)`, CAS: Get user profile.
- `cas_get_login_url(service, renew, gateway)`, CAS: Login URL.
- `cas_get_logout_url(service)`, CAS: Logout URL.

### Scim Service

- `scim_get_service_provider_config()`, SCIM: Get Service Provider Configuration.
- `scim_get_resource_types()`, SCIM: Get Resource Types.
- `scim_get_schemas()`, SCIM: Get Schemas.
- `scim_get_users(filter, start_index, count)`, SCIM: Get Users.
- `scim_get_user(id)`, SCIM: Get User by ID.
- `scim_create_user(user)`, SCIM: Create User.
- `scim_update_user(id, user)`, SCIM: Update User.
- `scim_patch_user(id, patch)`, SCIM: Patch User.
- `scim_delete_user(id)`, SCIM: Delete User.
- `scim_get_groups(filter, start_index, count)`, SCIM: Get Groups.
- `scim_get_group(id)`, SCIM: Get Group by ID.
- `scim_create_group(group)`, SCIM: Create Group.
- `scim_update_group(id, group)`, SCIM: Update Group.
- `scim_patch_group(id, patch)`, SCIM: Patch Group.
- `scim_delete_group(id)`, SCIM: Delete Group.

### Product Service

- `get_products()`, get all products.
- `get_product(id)`, get a product by id.
- `create_product(product)`, create a product.
- `update_product(product)`, update a product.
- `delete_product(product)`, delete a product.
- `get_products_by_owner(owner)`, get products by owner.

### Order Service

- `get_orders()`, get all orders.
- `get_order(id)`, get an order by id.
- `create_order(order)`, create an order.
- `update_order(order)`, update an order.
- `delete_order(order)`, delete an order.
- `get_orders_by_owner(owner)`, get orders by owner.
- `get_orders_by_user(user)`, get orders by user.
- `get_orders_by_product(product)`, get orders by product.

### Payment Service

- `get_payments()`, get all payments.
- `get_payment(id)`, get a payment by id.
- `create_payment(payment)`, create a payment.
- `update_payment(payment)`, update a payment.
- `delete_payment(payment)`, delete a payment.
- `get_payments_by_owner(owner)`, get payments by owner.
- `get_payments_by_user(user)`, get payments by user.
- `get_payments_by_order(order)`, get payments by order.
- `process_payment(payment_id, action)`, process a payment.
