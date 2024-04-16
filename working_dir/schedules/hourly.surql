-- ------------------------------
-- LOCK UNVERIFIED USERS AFTER 1W
-- ------------------------------

UPDATE user SET is_locked = true WHERE is_verified = false AND time_registered + 1w < time::now();

-- ------------------------------
-- DELETE EXPIRED VERIFICATION AFTER 1D
-- ------------------------------

DELETE verification_link WHERE time_created + 1d < time::now();

