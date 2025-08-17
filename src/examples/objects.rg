import Date
import Random
import UUID

local = Date.now # Date::LocalDate
utc = Date.utc # Date::UTCDate

int = Random.int
mut rand = Random.new 12345 # Random from seed, rand must be mutable
bool = rand.next_bool
float = rand.next_float

uuid = UUID.random # UUID, converted to string in console output

{ local, utc, int, bool, float, uuid }