import SharedTypes
import SwiftUI

enum Message {
    case message(Event)
}

@MainActor
class Model: ObservableObject {
    @Published var view = ViewModel(name: "", email: "")

    init() {
        update(msg: .message(.reset))
    }

    func update(msg: Message) {
        let reqs: [Request]

        switch msg {
        case let .message(m):
            reqs = try! [Request].bincodeDeserialize(input: CounterApp.processEvent(try! m.bincodeSerialize()))
        }

        for req in reqs {
            switch req.effect {
            case .render(_): view = try! ViewModel.bincodeDeserialize(input: CounterApp.view())
            }
        }
    }
}

struct InputView: View {
    @ObservedObject var model: Model
    @State private var name: String = ""
    @State private var email: String = ""

    var body: some View {
        VStack() {
            TextField("Enter your name", text: $name).padding(10)
            TextField("What's your email", text: $email).padding(10)
            Button("Update Details") {
                model.update(msg: .message(.setName(name)))
                model.update(msg: .message(.setEmail(email)))
            }.padding(20)
        }
    }
}

struct UserView: View {
    @ObservedObject var model: Model
    
    var body: some View {
        VStack {
            InputView(model: model)
            Text("Good Day,")
            Text("Name: " + model.view.name)
            Text("Email: " + model.view.email)
        }
    }
}

struct UserView_Previews: PreviewProvider {
    static var previews: some View {
        UserView(model: Model())
    }
}
