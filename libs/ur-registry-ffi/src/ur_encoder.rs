use crate::response::{PtrResponse, Response};
use ur::Encoder;

#[no_mangle]
pub extern "C" fn ur_encoder_next_part(ur_encoder: &mut Encoder) -> PtrResponse {
    match ur_encoder.next_part() {
        Ok(v) => {
            // For single-part URs, we might want to strip the "1-1" part numbering
            // to maintain backward compatibility, but ur 0.3.x doesn't expose
            // fragment_count() anymore. The client will get the full UR with
            // part numbering (e.g., "ur:bytes/1-1/...") which should still work.
            Response::success_string(v)
        },
        Err(e) => Response::error(e.to_string()),
    }
    .c_ptr()
}
