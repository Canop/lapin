
use {
    anyhow::Result,
    crate::{
        io::W,
        pos::ScreenPos,
        screen::*,
    },
    crossterm::{
        terminal::ClearType,
    },
    super::{
        pen::*,
        ink::*,
        inkwell::*,
    },
    termimad::{
        Area,
    },
};

/// a place on the screen on which you click to
/// select a shape
#[derive(Debug)]
struct ShapeWell {
    area: Area,
    shape: PenShape,
}

pub struct PenPanel<'s> {
    pen: &'s mut Pen,
    screen: &'s Screen,
    inkwells: Vec<InkWell>,
    shapewells: Vec<ShapeWell>,
}

impl<'s> PenPanel<'s> {
    pub fn new(
        pen: &'s mut Pen,
        screen: &'s Screen,
    ) -> Self {
        let inkwells = Vec::new(); // this will be filled when drawing
        let shapewells = Vec::new(); // this will be filled when drawing
        Self {
            pen,
            screen,
            inkwells,
            shapewells,
        }
    }

    fn draw_shape_pen_panel(&mut self, w: &mut W) -> Result<()> {
        let cs = &self.screen.skin.editor.paragraph.compound_style;
        let area = &self.screen.areas.pen_panel;
        let width = 8;
        let x = area.left + area.width - width;
        let mut y = area.top;
        for &shape in PEN_SHAPES {
            self.screen.goto(w, x, y)?;
            if self.pen.shape == shape {
                if self.pen.shape_started() {
                    cs.queue_str(w, "▸▸")?;
                } else {
                    cs.queue_str(w, "▸ ")?;
                }
            } else {
                cs.queue_str(w, "  ")?;
            }
            cs.queue(w, format!("{:?}", shape))?;
            self.shapewells.push(ShapeWell {
                shape,
                area: Area::new(x, y, width, 1),
            });
            y += 1;
        }
        Ok(())
    }

    pub fn draw(&mut self, w: &mut W) -> Result<()> {
        let area = &self.screen.areas.pen_panel;
        let cs = &self.screen.skin.editor.paragraph.compound_style;
        self.inkwells.clear();
        self.shapewells.clear();

        // clear first line
        self.screen.goto(w, 0, area.top)?;
        cs.clear(w, ClearType::UntilNewLine)?;

        // clear line below inkwells because we'll draw the marks
        self.screen.goto(w, 0, area.top + 2)?;
        cs.clear(w, ClearType::UntilNewLine)?;

        // Terrains
        let mut sp = ScreenPos::new(0, area.top + 1);
        sp.goto(w)?;
        self.inkwells.extend(draw_inkwells(
            w,
            self.screen,
            &mut sp,
            " Terrain:",
            TERRAIN_INKS,
            self.pen.ink,
        )?);

        // Items
        self.inkwells.extend(draw_inkwells(
            w,
            self.screen,
            &mut sp,
            " Item:",
            ITEM_INKS,
            self.pen.ink,
        )?);

        // Actors
        self.inkwells.extend(draw_inkwells(
            w,
            self.screen,
            &mut sp,
            " Actor:",
            ACTOR_INKS,
            self.pen.ink,
        )?);

        cs.clear(w, ClearType::UntilNewLine)?;

        self.draw_shape_pen_panel(w)?;

        Ok(())
    }

    //#[allow(dead_code)]
    //pub fn circle_around(&mut self, x: u16, y: u16) -> Result<()> {
    //    self.screen.goto(w, x-1, y-1)?;
    //    self.screen.skin.editor.paragraph.compound_style.queue_bg(w)?;
    //    w.queue(SetForegroundColor(self.screen.skin.editor_circle))?;
    //    write!(w, "▗")?;
    //    write!(w, "▄")?;
    //    write!(w, "▖")?;
    //    self.screen.goto(w, x-1, y)?;
    //    write!(w, "▐")?;
    //    self.screen.goto(w, x+1, y)?;
    //    write!(w, "▌")?;
    //    self.screen.goto(w, x-1, y+1)?;
    //    write!(w, "▝")?;
    //    write!(w, "▀")?;
    //    write!(w, "▘")?;
    //    Ok(())
    //}

    pub fn click(&mut self, sp: ScreenPos) {
        debug!("pen_panel click {:?}", sp);
        for shapewell in &self.shapewells {
            if sp.is_in(&shapewell.area) {
                self.pen.set_shape(shapewell.shape);
                return;
            }
        }
        for inkwell in &self.inkwells {
            if inkwell.sp == sp {
                self.pen.set_ink(inkwell.ink);
                return;
            }
        }
    }

}
