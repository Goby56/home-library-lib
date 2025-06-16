CREATE TABLE "Session" (
    "id" TEXT NOT NULL UNIQUE,
    "secret_hash" TEXT NOT NULL,
    "created_at" INTEGER NOT NULL,
    "user" INTEGER NOT NULL,
    FOREIGN KEY("user") REFERENCES "User"("id") ON DELETE CASCADE
);
