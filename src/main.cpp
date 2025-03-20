#include <llvm/IR/LLVMContext.h>
#include <llvm/IR/Module.h>
#include <llvm/IR/Function.h>
#include <llvm/IR/IRBuilder.h>
#include <llvm/Support/raw_ostream.h>
#include <llvm/IR/Verifier.h>

int main()
{
    // Set up the LLVM context and module
    llvm::LLVMContext context;
    llvm::Module module("neSia", context);
    llvm::IRBuilder<> builder(context);

    // Create the function signature: int main()
    llvm::FunctionType *funcType = llvm::FunctionType::get(
        llvm::Type::getInt32Ty(context),
        false);

    llvm::Function *mainFunction = llvm::Function::Create(
        funcType,
        llvm::Function::ExternalLinkage,
        "main",
        module);

    // Create a basic block and set the insertion point
    llvm::BasicBlock *entry = llvm::BasicBlock::Create(context, "entry", mainFunction);
    builder.SetInsertPoint(entry);

    // Declare two integer variables 'x' and 'y'
    llvm::AllocaInst *x = builder.CreateAlloca(llvm::Type::getInt32Ty(context), nullptr, "x");
    llvm::AllocaInst *y = builder.CreateAlloca(llvm::Type::getInt32Ty(context), nullptr, "y");

    // Assign values to 'x' and 'y'
    builder.CreateStore(llvm::ConstantInt::get(llvm::Type::getInt32Ty(context), 10), x); // x = 10
    builder.CreateStore(llvm::ConstantInt::get(llvm::Type::getInt32Ty(context), 20), y); // y = 20

    // Load the values of 'x' and 'y' and add them together
    llvm::Value *loadedX = builder.CreateLoad(llvm::Type::getInt32Ty(context), x, "loadedX");
    llvm::Value *loadedY = builder.CreateLoad(llvm::Type::getInt32Ty(context), y, "loadedY");

    llvm::Value *sum = builder.CreateAdd(loadedX, loadedY, "sum");

    // Return the sum (simulating a return from main())
    builder.CreateRet(sum);

    // Verify the function and the module
    llvm::verifyFunction(*mainFunction);
    llvm::verifyModule(module);

    // Print the module to stdout (LLVM IR)
    module.print(llvm::outs(), nullptr);

    return 0;
}
