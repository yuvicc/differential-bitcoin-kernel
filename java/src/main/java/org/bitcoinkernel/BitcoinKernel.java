package org.bitcoinkernel;

import static org.bitcoinkernel.BitcoinKernelBindings.*

public class BitcoinKernel implements AutoCloseable {
    
    
    @override
    public void close(){}
}

public class ChainstateManagerOptions implements AutoCloseable {
    
    public ChainstateManagerOptions(){}
    
    public void SetWorkerThreads(int worker_threads) {}
    
    public ChainstateManagerOptions SetWipeDbs(){}
    
    public ChainstateManagerOptions SetBlockTreeDbInMemory(){}
    
    public ChainstateManagerOptions SetChainstateMemoryIbDb(){}
    
    @override
    public void close(){}
}

public class ChainMan implements AutoCloseable {
    
    public ChainMan(){}
    
    public 
    
    @override
    public void close(){}
}

public class ContextManager implements AutoCloseable {
    
    public ContextManager(){}
    
    @override
    public void close(){}
}

public class Logger implements AutoCloseable {
    
    public Logger(){}
    
    @override
    public void close(){}
}

public class NotificationManager implements AutoCloseable {
    
    public NotificationManager(){}
    
    @override
    public void close(){}
}

public class KernelUtil implements AutoCloseable {
    
    public KernelUtil(){}
    
    @override
    public void close(){}
}