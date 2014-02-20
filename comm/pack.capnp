@0xb9dbc3cc737c493c;

struct PackData {
	
	struct Message {
		target :union {
			public @0 :Void;
			announcement @1 :Void;
			private @2 :Text;
		}
		message @3 :Text;
	}

	union {
		pubkey @0 :Data;
		login :group {
			name @1 :Text;
		}
		message @2 :Message;
		responseCode @3 :UInt8;
		quit @4 :Text;
	}
}

struct Pack {
	nonce @0 :Data;
	data @1 :Data;
}