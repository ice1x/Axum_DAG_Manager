# Security Policy

## Reporting Security Issues

If you discover a security vulnerability, please email [your-email] or open a private security advisory on GitHub.

## Known Advisory Exceptions

### RUSTSEC-2023-0071: RSA Marvin Attack (Medium Severity)

**Status**: Acknowledged, Not Applicable

**Affected Crate**: `rsa 0.9.9`

**Dependency Path**:
```
rsa 0.9.9
└── sqlx-mysql 0.8.6
    └── sqlx-macros 0.8.6
        └── sqlx 0.8.6
            └── dag-service 0.1.0
```

**Rationale for Exception**:

1. **Not Used at Runtime**: This project only uses PostgreSQL. The MySQL driver (`sqlx-mysql`) is pulled in as a compile-time dependency for `sqlx` macros, but is NOT included in the final binary.

2. **Vulnerability Scope**: The RSA vulnerability affects MySQL authentication protocol, which we don't use. Our runtime dependencies only include `sqlx-postgres`.

3. **Mitigation**:
   - We explicitly use `default-features = false` for sqlx
   - Only PostgreSQL-specific features are enabled
   - The vulnerable code path is never executed

4. **Upstream Status**: This is a known limitation of sqlx 0.8 where the `macros` feature requires all database drivers at compile time for type checking, but they're not linked into the final binary.

**Verification**:
```bash
# Check runtime dependencies (MySQL should not appear)
cargo tree --edges normal | grep mysql

# Verify only PostgreSQL is used
grep -r "mysql\|MySQL" src/  # Should return nothing
```

**Resolution Plan**:
- Monitor sqlx updates for better feature granularity
- Consider migrating to sqlx 0.9+ when available with improved feature separation
- Alternative: Remove `macros` feature and use `query()` instead of `query!()` (requires more code changes)

## Security Best Practices

This project follows these security practices:

- ✅ Regular dependency updates via Dependabot
- ✅ Automated security audits in CI/CD
- ✅ Minimal dependency footprint (only PostgreSQL driver)
- ✅ Input validation on all API endpoints
- ✅ Proper error handling without information leakage
- ✅ Use of prepared statements (via sqlx) to prevent SQL injection

## Dependency Policy

- We only include dependencies that are actively maintained
- Security vulnerabilities are addressed within 30 days of disclosure
- Compile-time only dependencies are acceptable if properly isolated
