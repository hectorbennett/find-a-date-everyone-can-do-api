# find-a-date-everyone-can-do-api

cargo watch -x run

Connect to db:

```
PGPASSWORD="" psql -U doadmin -h db-postgresql-lon1-31699-do-user-2652264-0.b.db.ondigitalocean.com -p 25060 -d defaultdb --set=sslmode=disable
```

CREATE TABLE events (
	id UUID DEFAULT uuid_generate_v4 (),
	name TEXT NOT NULL,
	creation_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    modification_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	PRIMARY KEY (id)
);

CREATE TABLE users (
    id UUID DEFAULT uuid_generate_v4 (),
    event_id UUID NOT NULL,
	name TEXT NOT NULL,
    PRIMARY KEY(id),
    CONSTRAINT fk_event
      FOREIGN KEY(event_id) 
	  REFERENCES events(id)
);

CREATE TABLE dates (
    user_id UUID NOT NULL,
	date DATE NOT NULL,
    CONSTRAINT fk_user
      FOREIGN KEY(user_id) 
	  REFERENCES users(id)
);
