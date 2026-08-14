// A module whose name is the same as the package's named address (as in `haneul::haneul`):
// importing it would silently shadow the address for the rest of the module.
module ImportConflict::ImportConflict {

    public fun pub_pkg() {
    }
}
