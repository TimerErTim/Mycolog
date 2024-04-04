-- ------------------------------
-- LOCK UNVERIFIED USERS AFTER 1W
-- ------------------------------

UPDATE user SET is_locked = true WHERE is_verified = false AND time_registered - time::now() > 1w;

