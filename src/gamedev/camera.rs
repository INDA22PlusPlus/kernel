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
    fn scroll(&self) {}
}

// TODO: Research more about this OOP like structure, better ways to create rust like structs
pub struct BasicCamera {
    pos: Vec2<usize>,                   // Upper left corner, because of usize being non-negative
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

    // TODO: Figure out a way to make the stationary struct have
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

    pub fn add_pos(&mut self, pos: Vec2<usize>) {
        self.external_offset += pos;
    }

    pub fn add_offset(&mut self, offset: Vec2<usize>) {
        self.external_offset += offset;
    }
}

pub struct CameraStationary {
    basic_camera: BasicCamera,
}

impl UpdateCamera for CameraStationary {}

impl CameraStationary {
    pub fn new() -> Self {
        CameraStationary {
            basic_camera: BasicCamera::new()
        }
    }
}

pub struct CameraFollowing {
    basic_camera: BasicCamera,
    player: Player,
}

impl UpdateCamera for CameraFollowing {
    fn scroll(&self) {
        // self.basic_camera.pos
    }
}

pub struct CameraFollowingBorders {
    basic_camera: BasicCamera,
    player: Player,

    // Lower right corner, assuming (0,0) is the starting point in top left (Because of usize)
    borders: Vec2<usize>
}