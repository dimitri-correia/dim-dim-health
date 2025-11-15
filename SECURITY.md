# Security Policy

## Reporting Security Vulnerabilities

If you discover a security vulnerability in this project, please report it responsibly:

1. **DO NOT** open a public GitHub issue
2. Email the maintainers directly with details
3. Allow time for a fix to be developed before public disclosure

## Known Vulnerabilities

### Dependency Vulnerabilities

#### RSA Timing Attack (RUSTSEC-2023-0071)
- **Severity**: Medium (CVSS 5.9)
- **Affected Package**: rsa 0.9.9 (indirect dependency via sqlx-mysql)
- **Status**: No fixed upgrade available
- **Impact**: Potential key recovery through timing sidechannels (Marvin Attack)
- **Mitigation**: The application uses PostgreSQL as its primary database, not MySQL, so the vulnerable code path is not executed in production. Monitoring for upstream updates.

## Security Recommendations

### Critical Improvements Needed

1. **Rate Limiting** ⚠️ CRITICAL
   - No rate limiting is currently implemented on authentication endpoints
   - Endpoints at risk: login, registration, password reset
   - Recommended: Implement rate limiting using the existing Redis infrastructure
   - Impact: Without this, the application is vulnerable to brute force attacks and credential stuffing

2. **CORS Configuration** ⚠️ HIGH
   - No CORS policy is currently configured
   - Impact: Application is vulnerable to Cross-Site Request Forgery (CSRF) attacks
   - Recommended: Add tower-http CORS middleware with strict origin whitelist

3. **Security Headers** ⚠️ HIGH
   - Missing security headers in HTTP responses
   - Recommended headers:
     - `X-Content-Type-Options: nosniff`
     - `X-Frame-Options: DENY`
     - `X-XSS-Protection: 1; mode=block`
     - `Strict-Transport-Security: max-age=31536000; includeSubDomains`

### Medium Priority Improvements

4. **Error Handling**
   - Multiple `unwrap()` calls found in codebase (30+ instances)
   - Potential for panics leading to service disruption
   - Recommended: Replace with proper error handling using `Result` types

5. **Timing Attack Mitigation**
   - User enumeration possible through timing differences in login/registration
   - Recommended: Implement constant-time responses for authentication flows

### Configuration Security

6. **Production Configuration**
   - Ensure JWT secrets are:
     - Generated using cryptographically secure random number generators
     - At least 256 bits (32 bytes) in length
     - Never committed to version control
     - Rotated periodically
   - Use environment variables for all secrets in production
   - The dev configuration file contains a hardcoded secret which is acceptable for development only

## Security Features Currently Implemented ✅

1. **Authentication & Authorization**
   - JWT-based authentication with 15-minute token expiration
   - Refresh token rotation preventing token reuse
   - Security breach detection for reused refresh tokens (all user tokens invalidated)
   - Email verification required for sensitive operations
   - Three-tier authentication middleware: RequireAuth, OptionalAuth, RequireVerifiedAuth

2. **Password Security**
   - bcrypt password hashing with DEFAULT_COST (currently 12)
   - Password validation (minimum 8 characters)
   - Secure password reset flow with 1-hour token expiration
   - All reset tokens invalidated after successful password change

3. **SQL Injection Prevention**
   - SeaORM used for all database operations (parameterized queries)
   - No raw SQL queries in codebase

4. **Token Security**
   - Cryptographically secure token generation using UUID v4
   - Proper token expiration and cleanup
   - Token storage in database with expiration tracking

5. **Email Verification**
   - Email verification required before sensitive operations
   - 2-hour expiration on verification tokens
   - Automatic cleanup of expired tokens

## Security Best Practices for Development

1. **Never commit secrets** to version control
2. **Use environment variables** for configuration in production
3. **Keep dependencies updated** - run `cargo audit` regularly
4. **Test authentication flows** thoroughly
5. **Review code changes** for security implications
6. **Use HTTPS** in production (assumed)
7. **Monitor logs** for suspicious activity

## Security Audit History

- **2025-11-15**: Initial comprehensive security audit completed
  - 1 medium severity dependency vulnerability identified (no immediate fix available)
  - 3 high priority improvements recommended
  - 2 medium priority improvements recommended
  - Strong authentication system confirmed
  - SQL injection protection verified

## Contact

For security concerns, please contact the repository maintainer.
