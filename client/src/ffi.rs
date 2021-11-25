use crate::internal;
use byond::byond;

byond!(start_v8: path; {
    internal::start_v8(path);

    ""
});

byond!(stop_v8; {
    internal::stop_v8();

    ""
});

byond!(execute_js: code; {
    internal::execute_js(code)
});
