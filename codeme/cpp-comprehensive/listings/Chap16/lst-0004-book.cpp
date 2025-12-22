
// https://godbolt.org/z/ennEoWPzb 
void function(const MyValue &paramRef) {
    MyValue localValue{"local"};
}

int main() {
    MyValue mvalue1{"mvalue1"};
    function( MyValue{"temp"} );
    function( mvalue1 );
    {
        MyValue mvalue2{"mvalue2"};
    }
}