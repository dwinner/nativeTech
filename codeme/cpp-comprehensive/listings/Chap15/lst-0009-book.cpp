
struct Component {
    Color getColor() const { return white; }
};
struct Window : public Component { };
struct MainWindow : public Window { };
struct Dialog : public Window { };
struct TextInput : public Component { };
struct Button : public Component {
    Color getColor() const  { return gray; }
};