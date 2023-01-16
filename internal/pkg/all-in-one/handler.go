package allinone

import "C"
import "unsafe"

//go:linkname goErrorHandler c.goErrorHandler
//export goErrorHandler
func goErrorHandler(size C.int, start *C.char) {
	dest := make([]byte, size)
	copy(dest, (*(*[1024]byte)(unsafe.Pointer(start)))[:size:size])

	println("Error from all in one runner: %s", string(dest))
}
