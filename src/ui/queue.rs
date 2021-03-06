use cursive::direction::Orientation;
use cursive::event::Key;
use cursive::traits::Boxable;
use cursive::traits::Identifiable;
use cursive::views::*;
use cursive::Cursive;

use std::sync::Arc;
use std::sync::Mutex;

use queue::Queue;
use track::Track;
use ui::splitbutton::SplitButton;
use ui::trackbutton::TrackButton;

pub struct QueueView {
    pub view: Option<BoxView<ScrollView<IdView<LinearLayout>>>>,
    queue: Arc<Mutex<Queue>>,
}

impl QueueView {
    pub fn new(queue: Arc<Mutex<Queue>>) -> QueueView {
        let queuelist = LinearLayout::new(Orientation::Vertical).with_id("queue_list");
        let scrollable = ScrollView::new(queuelist).full_screen();

        QueueView {
            view: Some(scrollable),
            queue: queue,
        }
    }

    fn cb_delete(cursive: &mut Cursive, queue: &mut Queue) {
        let view_ref: Option<ViewRef<LinearLayout>> = cursive.find_id("queue_list");
        if let Some(mut queuelist) = view_ref {
            let index = queuelist.get_focus_index();
            queue.remove(index);
            queuelist.remove_child(index);
        }
    }

    fn cb_play(cursive: &mut Cursive, queue: &mut Queue) {
        let view_ref: Option<ViewRef<LinearLayout>> = cursive.find_id("queue_list");
        if let Some(queuelist) = view_ref {
            let index = queuelist.get_focus_index();
            queue.play(index);
        }
    }

    fn create_button(&self, track: &Track) -> SplitButton {
        let mut button = TrackButton::new(&track);
        // 'd' deletes the selected track
        {
            let queue_ref = self.queue.clone();
            button.add_callback('d', move |cursive| {
                Self::cb_delete(
                    cursive,
                    &mut queue_ref.lock().expect("could not lock queue"),
                );
            });
        }

        // <enter> plays the selected track
        {
            let queue_ref = self.queue.clone();
            button.add_callback(Key::Enter, move |cursive| {
                Self::cb_play(
                    cursive,
                    &mut queue_ref.lock().expect("could not lock queue"),
                );
            });
        }
        button
    }

    pub fn repopulate(&self, cursive: &mut Cursive) {
        let view_ref: Option<ViewRef<LinearLayout>> = cursive.find_id("queue_list");
        if let Some(mut queuelist) = view_ref {
            while queuelist.len() > 0 {
                queuelist.remove_child(0);
            }

            let queue = self.queue.lock().expect("could not lock queue");
            for track in queue.iter() {
                let button = self.create_button(track);
                queuelist.add_child(button);
            }
        }
    }
}
