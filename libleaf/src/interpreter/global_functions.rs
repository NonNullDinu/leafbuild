mod functions;

#[inline]
pub(crate) fn get_global_functions() -> CallPool {
    functions::get_global_functions()
}