use conrod_core::widget_ids;

widget_ids! {
    pub struct Ids {
        // top row
        circle_button,
        rectangle_button,
        triangle_button,
        undo_button,
        redo_button,
        zoom_in_button,
        save_button,
        main_menu_button,

        // bottom row
        fixed_joint_button,
        rotating_joint_button,
        sliding_joint_button,
        text_button,
        paste_button,
        zoom_out_button,
        load_button,

        // special
        play_button,
        stop_button,

        shape_count,

        canvas,
        text,

        file,
        edit,
        view,
        extras,
    }
}
