#include <iostream> // For std::cout
#include <memory> // For std::weak_ptr, std::shared_ptr, std::make_shared
#include <string.h>
using namespace std; // dropping std::
struct Person;


struct Team{
    string name = "team name";
    shared_ptr<Person> goalKeeper;
    ~Team()
    {
        cout<<"Team " << name << " destructed." << endl;
    }
};
struct Person{
    string name = "person name";
    weak_ptr<Team> team; // This line is changed.
    ~Person()
    {
        cout<<"Person " << name << " destructed." << endl;
    }
};

int main(){
    auto Barca = make_shared<Team>();
    Barca->name = "Barca";
    auto Valdes = make_shared<Person>();
    Valdes->name = "Valdes";
    cout << "Barca assigned as Valdes as goal keeper" << endl;
    Barca->goalKeeper = Valdes;
    cout << "Valdes assigned as Barca to team" << endl;
    Valdes->team = Barca;
    cout << "ending the code" << endl;
    return 0;

}