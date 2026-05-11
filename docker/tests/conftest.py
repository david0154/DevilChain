import pytest
import time
import requests
import os

NODE = os.getenv("NODE_API", "http://localhost:8545")

def wait_for_service(url: str, retries: int = 30, delay: float = 2.0):
    for i in range(retries):
        try:
            r = requests.get(url, timeout=5)
            if r.status_code < 500:
                return True
        except Exception:
            pass
        print(f"  Waiting for {url} ({i+1}/{retries})...")
        time.sleep(delay)
    return False

@pytest.fixture(scope="session", autouse=True)
def wait_for_node():
    """Wait for blockchain node to be ready before running tests"""
    print("\nWaiting for DevilChain node...")
    ready = wait_for_service(f"{NODE}/api/status")
    if not ready:
        pytest.skip("DevilChain node not available")
    print("Node ready!")
