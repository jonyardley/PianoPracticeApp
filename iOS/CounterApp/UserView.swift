import SharedTypes
import SwiftUI

enum Message {
    case message(Event)
}

@MainActor
class Model: ObservableObject {
    @Published var view = ViewModel(name: "", email: "")

    init() {
        update(msg: .message(.none))
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

struct UserView: View {
    @ObservedObject var model: Model
    
    var body: some View {
        VStack {
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
