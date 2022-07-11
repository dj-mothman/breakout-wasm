use js_sys::{Reflect, Function};
use breakout::{BreakoutGame, Direction};
use wasm_bindgen::{JsValue, UnwrapThrowExt, prelude::Closure, JsCast};
use wasm_react::{c, h, Component, hooks::{use_state, use_effect, Deps, use_callback}, export_components, props::Style};
use web_sys::{window, KeyboardEvent};

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

		use_effect({
			let game = game.clone();
			move || {
				let tick_closure = Closure::new({
					let mut game = game.clone();
					move || {
						game.set(|mut game| {
							game.tick();
							game
						})
					}
				});

			let handle = window().unwrap_throw().set_interval_with_callback_and_timeout_and_arguments_0(
				tick_closure.as_ref().dyn_ref::<Function>().unwrap_throw(),
				250)
				.unwrap_throw();

			move || {
				drop(tick_closure);
				window().unwrap_throw().clear_interval_with_handle(handle);
			}
		}}, Deps::none());

		let handle_key_down = use_callback({
			let mut game = game.clone();
			move |evt: KeyboardEvent| {			
			let direction = match evt.code().as_ref() {
				"ArrowLeft" => Some(Direction::Left),
				"ArrowRight" => Some(Direction::Right),
				_ => None,
			};

			if let Some(direction) = direction {
				game.set(|mut game| {
					game.move_paddle(direction);
					game
				})
			}
		}}, Deps::none());

		h!(div)
			.tabindex(0)
			.autofocus(true)
			.on_keydown(&handle_key_down)
			.style(&Style::new()
				.display("inline-grid")
				.grid_template(format!("repeat({}, 1em) / repeat({}, 1em)", self.height, self.width))
				.border("1px solid grey"))
			.build(c![
			..game.value().pos_iter().map(|pos| {
				let ball = &game.value().ball;
				if pos.x == ball.position.x && pos.y == ball.position.y {
					return h!(div).style(&Style::new().text_indent("-.2em").margin_top("-.2em")).build(c!["⚫"])
				} 

				for b in 0..game.value().blocks.len() {
					let block = &game.value().blocks[b];
					for p in 0..block.positions.len() {
						let block_pos = &block.positions[p];
						if pos.x == block_pos.x as i8 && pos.y == block_pos.y as i8 {
							return h!(div).style(&Style::new().text_indent("-.2em").margin_top("-.2em")).build(c![block.color.to_string()])
						}
					}
				}

				for p in 0..game.value().player.len() {
					let player_block = &game.value().player[p];
					if pos.x == player_block.x && pos.y == player_block.y {
						return h!(div).style(&Style::new().text_indent("-.2em").margin_top("-.2em")).build(c!["⬛"])
					}
				}

				return h!(div).build(c![])
			})
		])
	}
}

export_components! { App }