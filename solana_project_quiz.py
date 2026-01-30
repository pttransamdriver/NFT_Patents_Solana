#!/usr/bin/env python3
"""
Solana Patent NFT Marketplace Quiz
Tests knowledge of Solana program architecture, PDAs, and execution flows
"""

import random
import sys

class SolanaProjectQuiz:
    def __init__(self):
        self.questions = [
            {
                "question": "When minting a patent NFT, which file contains the Rust program logic?",
                "options": ["programs/patent-nft/src/lib.rs", "app/sdk.ts"],
                "correct": 0,
                "explanation": "programs/patent-nft/src/lib.rs:52 (mint_patent_nft) contains the on-chain program logic"
            },
            {
                "question": "What PDA seeds are used to derive the patent registry account?",
                "options": ["[b'state']", "[b'patent', patent_hash]"],
                "correct": 1,
                "explanation": "Patent registry uses [b'patent', patent_hash] to ensure each patent can only be minted once"
            },
            {
                "question": "Which SDK function prepares the minting transaction?",
                "options": ["app/sdk.ts:147 (PatentNFTSDK.mintPatentNFT)", "app/sdk.ts:330 (NFTMarketplaceSDK.listNFT)"],
                "correct": 0,
                "explanation": "PatentNFTSDK.mintPatentNFT() derives PDAs and builds the minting transaction"
            },
            {
                "question": "What is the minimum minting fee required?",
                "options": ["0.025 SOL", "0.05 SOL"],
                "correct": 1,
                "explanation": "programs/patent-nft/src/lib.rs:53 verifies payment >= 0.05 SOL"
            },
            {
                "question": "Which program is called via CPI to create NFT metadata?",
                "options": ["SPL Token Program", "Metaplex Token Metadata Program"],
                "correct": 1,
                "explanation": "Metaplex Token Metadata Program is called to create NFT metadata and master edition"
            },
            {
                "question": "When listing an NFT, where is the NFT transferred to?",
                "options": ["Marketplace program account", "Escrow ATA owned by listing PDA"],
                "correct": 1,
                "explanation": "NFT is transferred to an Associated Token Account owned by the listing PDA for escrow"
            },
            {
                "question": "What percentage marketplace fee is collected on sales?",
                "options": ["2.5%", "5%"],
                "correct": 0,
                "explanation": "programs/nft-marketplace/src/lib.rs:102 calculates 2.5% platform fee"
            },
            {
                "question": "Which instruction handles buying an NFT from the marketplace?",
                "options": ["programs/nft-marketplace/src/lib.rs:97 (buy_nft)", "programs/nft-marketplace/src/lib.rs:52 (list_nft)"],
                "correct": 0,
                "explanation": "buy_nft() handles the purchase, fee distribution, and NFT transfer"
            },
            {
                "question": "What is PSP in the Solana version?",
                "options": ["An ERC-20 token", "An SPL Token"],
                "correct": 1,
                "explanation": "PSP (Patent Search Pennies) is an SPL Token on Solana, not ERC-20"
            },
            {
                "question": "How many PSP tokens equal 1 SOL by default?",
                "options": ["100 PSP", "1,000 PSP"],
                "correct": 1,
                "explanation": "Default exchange rate is 1 SOL = 1,000 PSP (configurable)"
            },
            {
                "question": "Which program handles multi-token payment for searches?",
                "options": ["programs/search-payment/src/lib.rs", "programs/psp-token/src/lib.rs"],
                "correct": 0,
                "explanation": "search-payment program supports SOL, USDC, and PSP payments"
            },
            {
                "question": "What Anchor constraint verifies account ownership?",
                "options": ["#[account(signer)]", "#[account(has_one = authority)]"],
                "correct": 1,
                "explanation": "has_one constraint verifies that an account field matches the expected value"
            },
            {
                "question": "How does Solana prevent reentrancy attacks?",
                "options": ["nonReentrant modifier", "Explicit account passing and Rust borrow checker"],
                "correct": 1,
                "explanation": "Solana's architecture and Rust's borrow checker prevent reentrancy without modifiers"
            },
            {
                "question": "What is the typical transaction fee on Solana?",
                "options": ["~$0.0005 (0.0005 SOL)", "~$50 (variable gas)"],
                "correct": 0,
                "explanation": "Solana has fixed, low fees around 0.0005 SOL (~$0.05)"
            },
            {
                "question": "How long does transaction finality take on Solana?",
                "options": ["~15 seconds", "~400ms"],
                "correct": 1,
                "explanation": "Solana achieves finality in ~400ms vs ~15 seconds on Ethereum"
            },
            {
                "question": "Which file contains the TypeScript SDK for frontend integration?",
                "options": ["app/sdk.ts", "tests/patent-nft.ts"],
                "correct": 0,
                "explanation": "app/sdk.ts (503 lines) contains all SDK classes for frontend integration"
            },
            {
                "question": "What command runs the Anchor test suite?",
                "options": ["npm test", "anchor test"],
                "correct": 1,
                "explanation": "anchor test builds, deploys to localnet, and runs tests"
            },
            {
                "question": "Where is the Anchor configuration stored?",
                "options": ["Anchor.toml", "anchor.config.js"],
                "correct": 0,
                "explanation": "Anchor.toml contains program IDs, cluster URLs, and build settings"
            },
            {
                "question": "What is the maximum supply of PSP tokens?",
                "options": ["1,000,000 PSP", "10,000,000 PSP"],
                "correct": 1,
                "explanation": "programs/psp-token/src/lib.rs sets max supply to 10 million PSP"
            },
            {
                "question": "Which Metaplex account type represents NFT ownership?",
                "options": ["Metadata Account", "Master Edition Account"],
                "correct": 1,
                "explanation": "Master Edition Account represents limited supply NFTs (supply = 0 for unique NFTs)"
            },
            {
                "question": "What happens if you try to mint the same patent twice?",
                "options": ["Second mint succeeds with new token", "Transaction fails - patent already exists"],
                "correct": 1,
                "explanation": "Patent registry PDA prevents duplicate minting - transaction fails if patent exists"
            },
            {
                "question": "How are program events emitted in Anchor?",
                "options": ["emit!(EventName { ... })", "event.emit(EventName { ... })"],
                "correct": 0,
                "explanation": "Anchor uses emit! macro to emit events to transaction logs"
            },
            {
                "question": "What is rent-exempt balance?",
                "options": ["Minimum balance to keep account alive", "Fee paid per transaction"],
                "correct": 0,
                "explanation": "Accounts must maintain minimum balance (rent-exempt) to avoid deletion"
            },
            {
                "question": "Which instruction can pause the PSP token program?",
                "options": ["programs/psp-token/src/lib.rs:pause()", "programs/psp-token/src/lib.rs:stop()"],
                "correct": 0,
                "explanation": "pause() instruction sets state.paused = true to disable critical functions"
            },
            {
                "question": "What is a Program Derived Address (PDA)?",
                "options": ["A user's wallet address", "A deterministic address derived from seeds"],
                "correct": 1,
                "explanation": "PDAs are deterministic addresses derived from seeds without private keys"
            },
            {
                "question": "Which program controls the escrow NFT during listing?",
                "options": ["The seller", "The listing PDA (marketplace program)"],
                "correct": 1,
                "explanation": "Listing PDA owns the escrow ATA, giving the program control over the NFT"
            },
            {
                "question": "What is Cross-Program Invocation (CPI)?",
                "options": ["Calling other programs from your program", "Calling frontend from program"],
                "correct": 0,
                "explanation": "CPI allows programs to call other programs (e.g., SPL Token, Metaplex)"
            },
            {
                "question": "How many programs are in the Solana project?",
                "options": ["2 programs", "4 programs"],
                "correct": 1,
                "explanation": "4 programs: patent-nft, psp-token, nft-marketplace, search-payment"
            },
            {
                "question": "What is the total lines of Rust code across all programs?",
                "options": ["~1,000 lines", "~1,808 lines"],
                "correct": 1,
                "explanation": "Total: 415 + 483 + 394 + 516 = 1,808 lines of Rust"
            },
            {
                "question": "Which file would you check to understand the complete minting flow?",
                "options": ["TEACHME.md - FLOW 1: Minting a Patent NFT", "README.md - Installation"],
                "correct": 0,
                "explanation": "TEACHME.md contains detailed execution flows showing all files and accounts involved"
            },
        ]
        
        self.score = 0
        self.total_questions = 0
    
    def run_quiz(self, num_questions=10):
        """Run the quiz with specified number of questions"""
        print("\n" + "="*70)
        print("🎓 SOLANA PATENT NFT MARKETPLACE QUIZ")
        print("="*70)
        print(f"\nYou will be asked {num_questions} random questions.")
        print("This quiz tests your knowledge of:")
        print("  • Solana program architecture")
        print("  • Program Derived Addresses (PDAs)")
        print("  • Anchor framework concepts")
        print("  • Execution flows and file locations")
        print("  • Solana vs Ethereum differences")
        print("\nGood luck!\n")
        
        # Randomly select questions
        selected_questions = random.sample(self.questions, min(num_questions, len(self.questions)))
        
        for i, q in enumerate(selected_questions, 1):
            print(f"\n{'─'*70}")
            print(f"Question {i}/{num_questions}:")
            print(f"\n{q['question']}\n")
            
            # Shuffle options
            options = list(enumerate(q['options']))
            random.shuffle(options)
            
            # Find correct answer in shuffled options
            correct_idx = next(idx for idx, (orig_idx, _) in enumerate(options) if orig_idx == q['correct'])
            
            # Display options
            for idx, (_, option) in enumerate(options):
                print(f"  {idx + 1}. {option}")
            
            # Get user answer
            while True:
                try:
                    answer = input(f"\nYour answer (1-{len(options)}): ").strip()
                    answer_idx = int(answer) - 1
                    if 0 <= answer_idx < len(options):
                        break
                    print(f"Please enter a number between 1 and {len(options)}")
                except (ValueError, KeyboardInterrupt):
                    print("\nQuiz interrupted.")
                    return
            
            # Check answer
            self.total_questions += 1
            if answer_idx == correct_idx:
                self.score += 1
                print("\n✅ Correct!")
            else:
                print(f"\n❌ Incorrect. The correct answer was: {q['options'][q['correct']]}")
            
            print(f"\n💡 Explanation: {q['explanation']}")
        
        # Final score
        self.show_results()
    
    def show_results(self):
        """Display final quiz results"""
        percentage = (self.score / self.total_questions * 100) if self.total_questions > 0 else 0
        
        print("\n" + "="*70)
        print("📊 QUIZ RESULTS")
        print("="*70)
        print(f"\nYou scored: {self.score}/{self.total_questions} ({percentage:.1f}%)\n")
        
        if percentage >= 90:
            print("🏆 Outstanding! You're a Solana expert!")
        elif percentage >= 75:
            print("🎉 Great job! You have a solid understanding of Solana development.")
        elif percentage >= 60:
            print("👍 Good work! Review TEACHME.md for areas to improve.")
        else:
            print("📚 Keep studying! Read TEACHME.md and try again.")
        
        print("\n" + "="*70)
        print("\n📖 Resources:")
        print("  • TEACHME.md - Detailed architecture and execution flows")
        print("  • FAQ_SOLANA.md - 60 questions covering all aspects")
        print("  • DEPLOYMENT.md - Deployment instructions")
        print("  • MIGRATION_GUIDE.md - Ethereum to Solana migration")
        print("\n")

def main():
    """Main entry point"""
    quiz = SolanaProjectQuiz()
    
    # Check for command line argument
    num_questions = 10
    if len(sys.argv) > 1:
        try:
            num_questions = int(sys.argv[1])
        except ValueError:
            print("Usage: python solana_project_quiz.py [num_questions]")
            sys.exit(1)
    
    quiz.run_quiz(num_questions)

if __name__ == "__main__":
    main()

