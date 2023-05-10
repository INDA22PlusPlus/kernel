use crate::gamedev::player::Player;
use crate::math::vec2::Vec2;

const EXTERNAL_SCREEN_SIZE: Vec2<usize> = Vec2::<usize>::new(640, 480);
const INTERNAL_SCREEN_SIZE: Vec2<usize> = Vec2::<usize>::new(480, 480);
const EXTERNAL_OFFSET: Vec2<usize> = (EXTERNAL_OFFSET - INTERNAL_SCREEN_SIZE) / 2;

pub enum CameraType {
    Stationary(CameraStationary),
    Following(CameraFollowing),                    // Likely not going to be used
    FollowingBorders(CameraFollowingBorders),        // Most likely to be used
    // Scrolling,                       // Most likely not going to be used
}

trait UpdateCamera {
    fn scroll(&self) {

    }
}

pub struct BasicCamera {
    pos: Vec2<usize>,
    internal_offset: Vec2<usize>,
    external_offset: Vec2<usize>,
    internal_screen_size: Vec2<usize>,
    external_screen_size: Vec2<usize>,
}

impl BasicCamera {
    pub fn new() -> BasicCamera {
        BasicCamera {
            pos: Vec2::<usize>::new(0, 0),
            internal_offset: Vec2::<usize>::new(0, 0),
            external_offset: EXTERNAL_OFFSET,
            internal_screen_size: INTERNAL_SCREEN_SIZE,
            external_screen_size: EXTERNAL_SCREEN_SIZE,
        }
    }

    pub fn from_pos(pos: Vec2<usize>) -> BasicCamera {
        BasicCamera {
            pos,
            internal_offset: Vec2::<usize>::new(0, 0),
            external_offset: EXTERNAL_OFFSET,
            internal_screen_size: INTERNAL_SCREEN_SIZE,
            external_screen_size: EXTERNAL_SCREEN_SIZE,
        }
    }

    pub fn from_offset(offset: Vec2<usize>) -> BasicCamera {
        BasicCamera {
            pos: Vec2::<usize>::new(0, 0),
            internal_offset: offset,
            external_offset: EXTERNAL_OFFSET,
            internal_screen_size: INTERNAL_SCREEN_SIZE,
            external_screen_size: EXTERNAL_SCREEN_SIZE,
        }
    }

    pub fn from_pos_and_offset(pos: Vec2<usize>, offset: Vec2<usize>) -> BasicCamera {
        BasicCamera {
            pos,
            internal_offset: offset,
            external_offset: EXTERNAL_OFFSET,
            internal_screen_size: INTERNAL_SCREEN_SIZE,
            external_screen_size: EXTERNAL_SCREEN_SIZE,
        }
    }

    pub fn set_pos(&mut self, pos: Vec2<usize>) {
        self.pos = pos;
    }

    pub fn set_offset(&mut self, offset: Vec2<usize>) {
        self.internal_offset = offset;
    }

    pub fn set_external_offset(&mut self, offset: Vec2<usize>) {
        self.external_offset = offset;
    }

    pub fn set_internal_screen_size(&mut self, size: Vec2<usize>) {
        self.internal_screen_size = size;
    }

    pub fn set_external_screen_size(&mut self, size: Vec2<usize>) {
        self.external_screen_size = size;
    }
}

pub struct CameraStationary {}

pub struct CameraFollowing {}

pub struct CameraFollowingBorders {}