# Gmail Setup Guide for DimDim Health

This guide will help you set up a Gmail account (like `dimdimhealth@gmail.com`) to send emails from your DimDim Health application.

## Prerequisites

- A Gmail account (you can create one at https://accounts.google.com/signup)
- Access to your Gmail account settings

## Step 1: Create a Gmail Account

1. Go to https://accounts.google.com/signup
2. Create a new Gmail account (e.g., `dimdimhealth@gmail.com`)
3. Complete the account verification process

## Step 2: Enable 2-Step Verification

Gmail requires 2-Step Verification to be enabled before you can create App Passwords.

1. Go to your Google Account settings: https://myaccount.google.com/
2. Navigate to **Security** in the left sidebar
3. Under "How you sign in to Google," click on **2-Step Verification**
4. Follow the prompts to set up 2-Step Verification (you'll need a phone number)

## Step 3: Create an App Password

Since Google doesn't allow regular passwords for third-party apps, you need to create an "App Password":

1. Go to your Google Account settings: https://myaccount.google.com/
2. Navigate to **Security** in the left sidebar
3. Under "How you sign in to Google," click on **2-Step Verification** (make sure it's enabled)
4. Scroll down and click on **App passwords** (you might need to sign in again)
5. In the "App passwords" page:
   - Select **Mail** for the app
   - Select **Other (Custom name)** for the device
   - Enter a name like "DimDim Health Worker"
   - Click **Generate**
6. Google will show you a 16-character password like `xxxx xxxx xxxx xxxx`
7. **IMPORTANT**: Copy this password immediately - you won't be able to see it again!

## Step 4: Configure Your Application

1. Open the configuration file: `config/dev.toml`
2. Update the Gmail settings:

```toml
gmail_email = "dimdimhealth@gmail.com"
gmail_password = "xxxx xxxx xxxx xxxx"  # Your 16-character App Password
```

**Note**: Replace `dimdimhealth@gmail.com` with your actual Gmail address and paste the App Password you generated (you can include or remove the spaces - both work).

## Step 5: Test the Email Functionality

1. Start your application (worker service)
2. Trigger an email-sending action (e.g., user registration)
3. Check the logs to verify the email was sent successfully
4. Check the recipient's inbox to confirm delivery

## Troubleshooting

### Common Issues and Solutions

#### 1. "Authentication failed" error
- **Cause**: Wrong email or App Password
- **Solution**: Double-check your `gmail_email` and `gmail_password` in `config/dev.toml`
- Make sure you're using the App Password, not your regular Gmail password

#### 2. "2-Step Verification required" error
- **Cause**: 2-Step Verification is not enabled on your account
- **Solution**: Follow Step 2 above to enable 2-Step Verification

#### 3. "Less secure app access" message
- **Cause**: This is an old security feature that's no longer applicable
- **Solution**: Use App Passwords instead (Step 3)

#### 4. Emails going to spam
- **Cause**: Gmail's spam filters or recipient's email provider
- **Solution**: 
  - Ask recipients to mark your emails as "Not Spam"
  - Consider setting up SPF, DKIM, and DMARC records if you have a custom domain
  - Start with a low email volume to build reputation

#### 5. "Daily sending limit exceeded"
- **Cause**: Gmail has daily sending limits (typically 500 emails/day for regular accounts)
- **Solution**: 
  - For regular Gmail: Limit your sending to stay within the quota
  - For higher volume: Consider upgrading to Google Workspace or using a dedicated email service

## Security Best Practices

1. **Never commit credentials to Git**
   - The `config/dev.toml` file contains placeholder values
   - Always use environment-specific configuration files that are gitignored
   - Consider using environment variables for sensitive data in production

2. **Rotate App Passwords regularly**
   - If you suspect a password has been compromised, revoke it and create a new one
   - You can manage App Passwords at https://myaccount.google.com/apppasswords

3. **Use a dedicated email account**
   - It's recommended to use a dedicated Gmail account for your application
   - This separates your application's emails from personal emails

4. **Monitor your account**
   - Check your Google Account activity regularly
   - Set up alerts for suspicious activity

## Gmail Sending Limits

Be aware of Gmail's sending limits:

- **Regular Gmail Account**: 500 emails per day
- **Google Workspace Account**: 2,000 emails per day per user

If you need to send more emails, consider:
- Using Google Workspace (paid)
- Using a dedicated email service like SendGrid, Mailgun, or AWS SES

## Production Configuration

For production environments:

1. Create a separate configuration file (e.g., `config/prod.toml`)
2. Use environment variables to inject sensitive credentials:
   ```toml
   gmail_email = "${GMAIL_EMAIL}"
   gmail_password = "${GMAIL_APP_PASSWORD}"
   ```
3. Never commit production credentials to version control
4. Use secure secret management solutions (AWS Secrets Manager, HashiCorp Vault, etc.)

## Additional Resources

- [Google App Passwords Documentation](https://support.google.com/accounts/answer/185833)
- [Gmail SMTP Settings](https://support.google.com/mail/answer/7126229)
- [2-Step Verification Setup](https://support.google.com/accounts/answer/185839)

## Support

If you encounter issues not covered in this guide, please:
1. Check the application logs for detailed error messages
2. Verify all configuration settings
3. Ensure your Gmail account is properly set up with 2-Step Verification and App Passwords
