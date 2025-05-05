
use sdl2::rect::Rect;


pub fn clamp_to_area(rect: &mut Rect, area: &Rect) {

    if !area.contains_rect(*rect) {

        let (our_w, our_h) = rect.size();
        let (their_w, their_h) = area.size();

        if our_w > their_w || our_h > their_h {
            rect.center_on(area.center());
        } else {

            let (our_top, our_right, our_bottom, our_left) = (
                rect.top(),
                rect.right(),
                rect.bottom(),
                rect.left(),
            );

            let (their_top, their_right, their_bottom, their_left) = (
                area.top(),
                area.right(),
                area.bottom(),
                area.left(),
            );

            if our_top < their_top {
                rect.set_y(their_top);
            } else if our_bottom > their_bottom {
                rect.set_bottom(their_bottom);
            }

            if our_left < their_left {
                rect.set_x(their_left);
            } else if our_right > their_right {
                rect.set_right(their_right);
            }

        }

    }

}
