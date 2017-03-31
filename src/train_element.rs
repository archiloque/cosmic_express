use coordinates;
use train_element_content;
use train_element_status;

struct TrainElement {
    status : TrainElementStatus,
    content: TrainElementContent,
    head: bool,
    coordinates: Coordinates,
}