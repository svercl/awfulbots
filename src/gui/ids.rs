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

        part_count_text,

        part_canvas,
        part_name_label,
        part_delete_button,
        part_cut_button,
        part_copy_button,
        part_paste_button,
        part_rotate_button,
        part_density_text,
        part_density_slider,
        part_collides_toggle,
        part_collides_text,
        part_camera_focus_toggle,
        part_camera_focus_text,
        part_undraggable_toggle,
        part_undraggable_text,
        part_fixate_toggle,
        part_fixate_text,
        part_change_color_button,
        part_move_to_front_button,
        part_move_to_back_button,
        part_show_outlines_toggle,
        part_show_outlines_text,
        part_outlines_behind_toggle,
        part_outlines_behind_text,


        canvas,
        text,

        file,
        edit,
        view,
        extras,
    }
}
