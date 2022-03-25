-- This file should undo anything in `up.sql`
ALTER TABLE member
    DROP COLUMN modification_date;