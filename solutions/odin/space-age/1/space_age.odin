package space_age

Planet :: enum {
	Mercury,
	Venus,
	Earth,
	Mars,
	Jupiter,
	Saturn,
	Uranus,
	Neptune,
}

age :: proc(planet: Planet, seconds: int) -> f64 {
	seconds := f64(seconds) / 31557600

	switch planet {
	case .Mercury:
		return seconds / 0.2408467
	case .Venus:
		return seconds / 0.61519726
	case .Earth:
		return seconds
	case .Mars:
		return seconds / 1.8808158
	case .Jupiter:
		return seconds / 11.862615
	case .Saturn:
		return seconds / 29.447498
	case .Uranus:
		return seconds / 84.016846
	case .Neptune:
		return seconds / 164.79132
	}
	// Implement the procedure.
	return 0.0
}
