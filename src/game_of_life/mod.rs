#[allow(dead_code)]
use rand::Rng;
use raylib::{
    color::Color,
    prelude::{RaylibDraw, RaylibDrawHandle},
    RaylibHandle, RaylibThread,
};

use crate::engine::{
    Entity, EntityBuilder, Event, EventQueue, Game, GameState, System, SystemManager,
};

pub struct GameOfLife {
    cell_entities: Vec<Entity>,
    grid_entity: Entity,
    render_system: RenderSystem,
    grid_render_system: GridRenderSystem,
    input_system: InputSystem,
    cell_toggle_system: CellToggleSystem,
    grid_toggle_system: GridToggleSystem,
    system_manager: SystemManager,
    width: u32,
    height: u32,
    game_state: GameState,
    toggle_cells_event_queue: EventQueue,
    toggle_grid_event_queue: EventQueue,
}

impl GameOfLife {
    pub fn new(width: u32, height: u32) -> Self {
        let cell_entities = create_grid(width, height);
        let grid_entity = EntityBuilder::new(0)
            .with_component(Grid {
                width,
                height,
                cell_size: 10,
                show: false,
            })
            .build();

        let game_state = GameState::Paused;
        let mut system_manager = SystemManager::new();
        let toggle_cells_event_queue = EventQueue::new();
        let toggle_grid_event_queue = EventQueue::new();

        let life_system = LifeSystem;
        let render_system = RenderSystem;
        let grid_render_system = GridRenderSystem;
        let input_system = InputSystem::new(10);
        let cell_toggle_system = CellToggleSystem { width };
        let grid_toggle_system = GridToggleSystem;

        system_manager.add_system(life_system);

        Self {
            cell_entities,
            grid_entity,
            render_system,
            grid_render_system,
            input_system,
            cell_toggle_system,
            grid_toggle_system,
            system_manager,
            width,
            height,
            game_state,
            toggle_cells_event_queue,
            toggle_grid_event_queue,
        }
    }
}

impl Game for GameOfLife {
    fn handle_input(&mut self, rl: &RaylibHandle) {
        if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_SPACE) {
            match self.game_state {
                GameState::Playing => self.change_state(GameState::Paused),
                GameState::Paused => self.change_state(GameState::Playing),
            }
        }

        self.input_system.handle_input(
            rl,
            self.width,
            self.height,
            &mut self.toggle_cells_event_queue,
            &mut self.toggle_grid_event_queue,
        );
    }

    fn update(&mut self, delta_time: f32) {
        self.cell_toggle_system
            .process_events(&mut self.cell_entities, &mut self.toggle_cells_event_queue);

        self.grid_toggle_system
            .process_events(&mut self.grid_entity, &mut self.toggle_grid_event_queue);

        match self.game_state {
            GameState::Playing => {
                self.system_manager
                    .update(&mut self.cell_entities, delta_time);
            }
            _ => {}
        }
    }

    fn render(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::BLACK);

        self.render_system
            .render(&mut d, &self.cell_entities, self.width);
        self.grid_render_system.render(&mut d, &self.grid_entity);

        match self.game_state {
            GameState::Paused => {
                PauseRenderSystem.render(&mut d);
            }
            _ => {}
        }
    }

    fn change_state(&mut self, new_state: GameState) {
        self.game_state = new_state;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum CellState {
    Alive,
    Dead,
}

struct Cell {
    pub state: CellState,
}

struct Grid {
    pub width: u32,
    pub height: u32,
    pub cell_size: i32,
    pub show: bool,
}

fn create_grid(width: u32, height: u32) -> Vec<Entity> {
    let mut entities = Vec::new();
    let mut rng = rand::thread_rng();

    for x in 0..width {
        for y in 0..height {
            let cell_entity = EntityBuilder::new((x + y * width) as u32)
                .with_component(Cell {
                    state: if rng.gen_bool(0.5) {
                        CellState::Alive
                    } else {
                        CellState::Dead
                    },
                })
                .build();
            entities.push(cell_entity);
        }
    }

    entities
}

struct LifeSystem;

impl System for LifeSystem {
    fn update(&mut self, entities: &mut [Entity], _delta_time: f32) {
        let width = (entities.len() as f64).sqrt() as u32;

        let next_states: Vec<CellState> = entities
            .iter()
            .enumerate()
            .map(|(i, entity)| {
                if let Some(cell) = entity.get_component::<Cell>() {
                    let alive_neighbors = count_alive_neighbors(i, entities, width);
                    match (cell.state, alive_neighbors) {
                        (CellState::Alive, 2..=3) => CellState::Alive,
                        (CellState::Dead, 3) => CellState::Alive,
                        _ => CellState::Dead,
                    }
                } else {
                    CellState::Dead
                }
            })
            .collect();

        for (i, entity) in entities.iter_mut().enumerate() {
            if let Some(cell) = entity.get_component_mut::<Cell>() {
                cell.state = next_states[i];
            }
        }
    }
}

fn count_alive_neighbors(index: usize, entities: &[Entity], width: u32) -> usize {
    let mut count = 0;
    let height = width; // Assume square grid

    let x = index as u32 % width;
    let y = index as u32 / height;

    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }

            let nx = x as i32 + dx;
            let ny = y as i32 + dy;

            if nx < 0 || nx >= width as i32 || ny < 0 || ny >= height as i32 {
                continue;
            }

            let neighbor_index = (nx + ny * width as i32) as usize;
            if let Some(cell) = entities[neighbor_index].get_component::<Cell>() {
                if cell.state == CellState::Alive {
                    count += 1;
                }
            }
        }
    }

    count
}

struct RenderSystem;

impl RenderSystem {
    pub fn render(&self, rl: &mut RaylibDrawHandle, entities: &[Entity], width: u32) {
        let cell_size = 10i32;

        for (i, entity) in entities.iter().enumerate() {
            if let Some(cell) = entity.get_component::<Cell>() {
                let x = (i as u32 % width) * cell_size as u32;
                let y = (i as u32 / width) * cell_size as u32;

                let color = match cell.state {
                    CellState::Alive => Color::GREEN,
                    CellState::Dead => Color::BLACK,
                };

                rl.draw_rectangle(x as i32, y as i32, cell_size, cell_size, color);
            }
        }
    }
}

struct PauseRenderSystem;
impl PauseRenderSystem {
    pub fn render(&self, rl: &mut RaylibDrawHandle) {
        rl.draw_text("Paused", 400 - 48, 300 - 36, 36, Color::WHITE);
    }
}

struct GridRenderSystem;

impl GridRenderSystem {
    pub fn render(&self, rl: &mut RaylibDrawHandle, grid_entity: &Entity) {
        if let Some(grid) = grid_entity.get_component::<Grid>() {
            if grid.show {
                for x in 0..grid.width {
                    for y in 0..grid.height {
                        let x = x * grid.cell_size as u32;
                        let y = y * grid.cell_size as u32;

                        rl.draw_rectangle_lines(
                            x as i32,
                            y as i32,
                            grid.cell_size,
                            grid.cell_size,
                            Color::DARKGRAY,
                        );
                    }
                }
            }
        }
    }
}

struct ToggleCellEventData {
    x: u32,
    y: u32,
}

struct ToggleGridEventData;

struct InputSystem {
    cell_size: i32,
}
impl InputSystem {
    pub fn new(cell_size: i32) -> Self {
        Self { cell_size }
    }

    pub fn handle_input(
        &mut self,
        rl: &RaylibHandle,
        width: u32,
        height: u32,
        toggle_cells_event_queue: &mut EventQueue,
        toggle_grid_event_queue: &mut EventQueue,
    ) {
        if rl.is_mouse_button_pressed(raylib::consts::MouseButton::MOUSE_BUTTON_LEFT) {
            let mouse_position = rl.get_mouse_position();

            let cell_x = (mouse_position.x / self.cell_size as f32) as u32;
            let cell_y = (mouse_position.y / self.cell_size as f32) as u32;

            if cell_x < width && cell_y < height {
                let event = Event {
                    event_type: "toggle_cell".to_string(),
                    data: Box::new(ToggleCellEventData {
                        x: cell_x,
                        y: cell_y,
                    }),
                };
                toggle_cells_event_queue.push(event);
            }
        }

        if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_G) {
            let event = Event {
                event_type: "toggle_grid".to_string(),
                data: Box::new(ToggleGridEventData),
            };

            toggle_grid_event_queue.push(event);
        }
    }
}

struct CellToggleSystem {
    width: u32,
}

impl CellToggleSystem {
    fn process_events(&mut self, entities: &mut [Entity], event_queue: &mut EventQueue) {
        while let Some(event) = event_queue.pop() {
            if event.event_type == "toggle_cell" {
                if let Some(data) = event.data.downcast_ref::<ToggleCellEventData>() {
                    let index = (data.x + data.y * self.width) as usize;
                    if let Some(cell) = entities[index].get_component_mut::<Cell>() {
                        cell.state = match cell.state {
                            CellState::Alive => CellState::Dead,
                            CellState::Dead => CellState::Alive,
                        };
                    }
                }
            }
        }
    }
}

struct GridToggleSystem;
impl GridToggleSystem {
    fn process_events(&mut self, entity: &mut Entity, event_queue: &mut EventQueue) {
        while let Some(event) = event_queue.pop() {
            if event.event_type == "toggle_grid" {
                if let Some(grid) = entity.get_component_mut::<Grid>() {
                    grid.show = !grid.show;
                }
            }
        }
    }
}

pub fn run() {
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("Game of Life")
        .vsync()
        .build();

    let width = 80;
    let height = 80;

    let mut game = GameOfLife::new(width, height);

    while !rl.window_should_close() {
        let delta_time = rl.get_frame_time();

        game.handle_input(&rl);

        game.update(delta_time);

        game.render(&mut rl, &thread);
    }
}
