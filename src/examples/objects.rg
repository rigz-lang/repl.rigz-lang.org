import Date
import Random
import UUID

local = Date.now # Date::LocalDate
utc = Date.utc # Date::UTCDate

int = next_int # Random.next_int
mut rand = Random::Random.new 12345 # Random from seed, rand must be mutable
bool = rand.next_bool
float = rand.next_float

uuid = UUID.v4

{ local, utc, int, bool, float, uuid }