//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/aTTejrnWE
#include <memory>                       // unique_ptr
#include <string>
#include <iostream>
using std::unique_ptr; using std::string;
class Component { };                    // Dummy window hierarchy
class Label : public Component { };
class Textfield : public Component { };
class Button : public Component {
public:
    int id_;                            // ID to distinguish the buttons
    explicit Button(int id) : id_{id} {}
};
class Window { };
class MyDialog : public Window {
    string title_;
    unique_ptr<Label> lblFirstName_{new Label{}};         // lots of data fields
    unique_ptr<Textfield> txtFirstName_{new Textfield{}}; // … tied to the lifetime
    unique_ptr<Label> lblLastName_{new Label{}};          // … of the class
    unique_ptr<Textfield> txtLastName_{new Textfield{}};
    unique_ptr<Button> btnOk_{new Button{1}};
    unique_ptr<Button> btnCancel_{new Button{2}};
public:
    explicit MyDialog(const string& title) : title_{title} {}
    unique_ptr<Button> showModal()
        { return std::move(btnOk_); }    // Placeholder code; OK pressed
};
unique_ptr<MyDialog> createDialog() {
    return unique_ptr<MyDialog>{ // temporary value
        new MyDialog{"Please enter name"}};
}
int showDialog() {
    unique_ptr<MyDialog> dialog = createDialog();         // local variable
    unique_ptr<Button> pressed = dialog->showModal();     // return value
    return pressed->id_;
}
int main() {
    int pressed_id = showDialog();
    if(pressed_id == 1) {
        std::cout << "Thank you for pressing OK\n";
    }
}