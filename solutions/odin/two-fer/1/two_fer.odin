package two_fer
import "core:strings"

two_fer :: proc(name: string = "") -> string {
	name := name == "" ? "you" : name
	// Implement this procedure.

	return strings.concatenate({"One for ", name, ", one for me."})
}
