LET $SCHEMA_VERSION = "0.0.1";

DEFINE TABLE email SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD sender ON email TYPE string ASSERT string::is::email($after);

