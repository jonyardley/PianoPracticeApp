import SwiftUI

@main
struct CounterAppApp: App {
    var body: some Scene {
        WindowGroup {
            UserView(model: Model())
        }
    }
}
