class Demo:
    def __init__(self, wallet_address, avail_funds, ws_url):
        self.wallet_address = wallet_address
        self.avail_funds = avail_funds
        self.ws_url = ws_url

    def start_demo():
        pass


if __name__ == '__main__':
    wallet_address = "0x1234567890"
    avail_funds = 100
    demo_client = Demo(wallet_address, avail_funds)
    demo_client.avail_funds()