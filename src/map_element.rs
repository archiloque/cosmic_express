use train_element_content;

pub enum MapElement {
    Empty,
    Monster1InFilled,
    Monster1InEmpty,
    Monster2InFilled,
    Monster2InEmpty,
    Monster1OutFilled,
    Monster1OutEmpty,
    Monster2OutFilled,
    Monster2OutEmpty,
    Obstacle,
    Rail,
    Entry,
    Exit,
}

impl MapElement {
    fn can_inboard_monster(self) -> bool {
        return match self {
            MapElement::Monster1InFilled | MapElement::Monster2InFilled => true,
            _ => false,
        }
    }

    fn can_cross(self) -> bool {
        return match self {
            MapElement::Empty | MapElement::Exit => true,
            _ => false,
        }
    }

    fn monster(self) -> TrainElementContent {
        return match self {
            MapElement::Monster1InFilled => TrainElementContent::Monster1,
            MapElement::Monster2InFilled => TrainElementContent::Monster2,
            _ => false,
        }
    }

}