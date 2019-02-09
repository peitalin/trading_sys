-- Your SQL goes here
CREATE TABLE book_depth (
	id SERIAL PRIMARY KEY,
	event TEXT NOT NULL,
	event_time TIMESTAMP NOT NULL,
	symbol TEXT NOT NULL,
	update_first INT NOT NULL,
	update_final INT NOT NULL,
	bids JSONB[] NOT NULL,
	asks JSONB[] NOT NULL
)
