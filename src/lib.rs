use js_sys::Reflect;
use breakout::BreakoutGame;
use wasm_bindgen::JsValue;
use wasm_react::{c, h, Component, hooks::use_state, export_components, props::Style};

mod breakout;

pub struct App {
	width: u8,
	height: u8,
}

impl TryFrom<JsValue> for App {
	type Error = JsValue;

	fn try_from(value: JsValue) -> Result<App, JsValue> {
		Ok(App {
			width: Reflect::get(&value, &"width".into())?.as_f64().unwrap_or(20.0) as u8,
			height: Reflect::get(&value, &"height".into())?.as_f64().unwrap_or(10.0) as u8,
		})
	}
}

impl Component for App {
	fn render(&self) -> wasm_react::VNode {
		let game = use_state(|| BreakoutGame::new(self.width, self.height));

		h!(div)
			.style(&Style::new()
				.display("inline-grid")
				.grid_template(format!("repeat({}, 1em) / repeat({}, 1em)", self.height, self.width))
				.border("1px solid grey"))
			.build(c![
			..game.value().pos_iter().map(|pos| {
				for b in 0..game.value().blocks.len() {
					let block = &game.value().blocks[b];
					for p in 0..block.positions.len() {
						let block_pos = &block.positions[p];
						if pos.x == block_pos.x as i8 && pos.y == block_pos.y as i8 {
							return h!(div).build(c![block.color.to_string().chars().next().unwrap().to_string()])
						}
					}
				}

				return h!(div).build(c![])
			})
		])
	}
}

export_components! { App }