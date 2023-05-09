pub enum CameraType {
    Stationary(CameraStationary),
    Following(CameraFollowing),                    // Likely not going to be used
    FollowingBorders(CameraFollowingBorders),        // Most likely to be used
    // Scrolling,                       // Most likely not going to be used
}

pub struct CameraStationary {}

pub struct CameraFollowing {}

pub struct CameraFollowingBorders {}