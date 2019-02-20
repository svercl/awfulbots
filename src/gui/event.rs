#[derive(Debug)]
pub enum GuiEvent {
    CircleClicked,
    RectangleClicked,
    TriangleClicked,
    UndoClicked,
    RedoClicked,

    FixedJointClicked,
    RotatingJointClicked,
    SlidingJointClicked,
    TextClicked,
    PasteClicked,

    PlayClicked,

    FileMainMenuClicked,
    FileSaveClicked,
    FileLoadRobotClicked,
    FileLoadReplayClicked,
    FileLoadChallengeClicked,

    EditChangeSettingsClicked,
    EditClearAllClicked,
    EditUndoClicked,
    EditRedoClicked,
    EditCutClicked,
    EditCopyClicked,
    EditPasteClicked,
    EditDeleteClicked,
    EditMoveToFrontClicked,
    EditMoveToBackClicked,

    ViewZoomInClicked,
    ViewZoomOutClicked,

    ExtrasMirrorHorizontalClicked,
    ExtrasMirrorVerticalClicked,
    ExtrasScaleClicked,
    ExtrasThrustersClicked,
    ExtrasCannonClicked,
}
