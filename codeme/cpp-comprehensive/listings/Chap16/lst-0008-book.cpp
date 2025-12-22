
Database::Database(const char* filename)
    : db_{ db_open(filename) }
    {
        if(nullptr == db_) { // Error opening
            throw IllegalArgumentException("Error opening the DB");
        }
    }