import os
from mpmath import mp

def generate_pi_locally(digits):
    # Set the precision
    mp.dps = digits + 2
    print(f"[*] Generating {digits} digits of Pi locally...")
    
    # Calculate Pi
    pi_val = mp.pi
    
    # Convert to string and clean it up (remove '3.')
    return str(pi_val).replace(".", "")

def main():
    # Production Setup: No hardcoding, dynamic paths
    script_dir = os.path.dirname(os.path.abspath(__file__))
    output_dir = os.path.join(script_dir, "data")
    filepath = os.path.join(output_dir, "pi_million.txt")

    if not os.path.exists(output_dir):
        os.makedirs(output_dir)

    # Generate 1,000,000 digits
    pi_string = generate_pi_locally(1_000_000)

    with open(filepath, "w") as f:
        f.write(pi_string)
        f.flush()
    
    # Integrity Check: Pi starts 314159...
    if pi_string.startswith("314159"):
        print(f"\n[SUCCESS] Pi data generated and verified!")
        print(f"[*] Path: {filepath}")
    else:
        print("[ERROR] Something went wrong with the generation.")

if __name__ == "__main__":
    main()

