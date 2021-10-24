#[derive(Debug)]
pub enum Event {
    DistanceUpdated(Option<u32>),
}

#[derive(Debug)]
pub struct Config {
    pub min_distance: u32,
    pub max_distance: u32,
    pub num_leds: u32,
}

#[derive(Debug)]
pub struct Model {
    config: Config,
    distance: Option<u32>,
}

impl Model {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            distance: None,
        }
    }

    pub fn update(&mut self, event: Event) {
        match event {
            Event::DistanceUpdated(distance) => self.distance = distance,
        };
    }

    pub fn num_leds_on(&self) -> u32 {
        self.distance
            .map_or(0, |distance| self.num_leds_from_distance(distance))
    }

    fn num_leds_from_distance(&self, distance: u32) -> u32 {
        let range = self.config.max_distance - self.config.min_distance;
        ((distance - self.config.min_distance) * self.config.num_leds + (range / 2)) / range
    }
}
