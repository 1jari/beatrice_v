import beatrice_v_api

class MyPlatformImpl:
    def metadata(self):
        return beatrice_v_api.PlatformMetadata("ToyOS", "x86_64", "Test platform")

    def setup(self):
        print("Setting up")

    def set_driver(self, addr, signal):
        print(f"Set driver at {addr} to signal {signal}")

    def get_driver(self, addr):
        return 42

    def dispose(self):
        print("Disposed")

impl = MyPlatformImpl()
platform = beatrice_v_api.Platform(impl)
print(platform.metadata())
