use mlua::{ObjectLike, Table};
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};
use tracing::error;
use yazi_plugin::{LUA, elements::render_widgets};

use super::{completion, confirm, help, input, manager, pick, tasks, which};
use crate::Ctx;

pub(super) struct Root<'a> {
	cx: &'a Ctx,
}

impl<'a> Root<'a> {
	pub(super) fn new(cx: &'a Ctx) -> Self { Self { cx } }

	pub(super) fn reflow(area: Rect) -> mlua::Result<Table> {
		let area = yazi_plugin::elements::Rect::from(area);
		let root = LUA.globals().raw_get::<Table>("Root")?.call_method::<Table>("new", area)?;
		root.call_method("reflow", ())
	}
}

impl<'a> Widget for Root<'a> {
	fn render(self, area: Rect, buf: &mut Buffer) {
		let mut f = || {
			let area = yazi_plugin::elements::Rect::from(area);
			let root = LUA.globals().raw_get::<Table>("Root")?.call_method::<Table>("new", area)?;

			render_widgets(root.call_method("redraw", ())?, buf);
			Ok::<_, mlua::Error>(())
		};
		if let Err(e) = f() {
			error!("Failed to redraw the `Root` component:\n{e}");
		}

		manager::Preview::new(self.cx).render(area, buf);

		if self.cx.tasks.visible {
			tasks::Tasks::new(self.cx).render(area, buf);
		}

		if self.cx.pick.visible {
			pick::Pick::new(self.cx).render(area, buf);
		}

		if self.cx.input.visible {
			input::Input::new(self.cx).render(area, buf);
		}

		if self.cx.confirm.visible {
			confirm::Confirm::new(self.cx).render(area, buf);
		}

		if self.cx.help.visible {
			help::Help::new(self.cx).render(area, buf);
		}

		if self.cx.completion.visible {
			completion::Completion::new(self.cx).render(area, buf);
		}

		if self.cx.which.visible {
			which::Which::new(self.cx).render(area, buf);
		}
	}
}
