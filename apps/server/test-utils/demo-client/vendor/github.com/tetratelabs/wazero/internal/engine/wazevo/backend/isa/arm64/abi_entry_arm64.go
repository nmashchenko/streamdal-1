package arm64

// entrypoint enters the machine code generated by this backend which begins with the preamble generated by abiImpl.EmitGoEntryPreamble below.
// This implements wazevo.entrypoint, and see the comments there for detail.
func entrypoint(preambleExecutable, functionExecutable *byte, executionContextPtr uintptr, moduleContextPtr *byte, paramResultPtr *uint64, goAllocatedStackSlicePtr uintptr)

// afterGoFunctionCallEntrypoint enters the machine code after growing the stack.
// This implements wazevo.afterGoFunctionCallEntrypoint, and see the comments there for detail.
func afterGoFunctionCallEntrypoint(executable *byte, executionContextPtr uintptr, stackPointer uintptr)
