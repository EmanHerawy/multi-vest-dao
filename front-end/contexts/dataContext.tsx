declare let window: any;
import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useState,
} from "react";
import { toast } from "react-toastify";
import FundingDAO from "../contracts/multi_vest_dao.abi.json";
import { Proposal } from "../utils/interface";
/**init => init
        vote => vote
        execute => execute
        propose => propose
        stake => stake
        lockedTokens => user_balance
        minimumStake => minimum_stake
        totalProposals => total_proposals
        ProposalDetails => proposal_details
        totalMembers => total_members
        treasuryBalance => balance
        pause => pause_endpoint
        unpause => unpause_endpoint
        isPaused => paused_status */
interface DataContextProps {
  account: string;
  loading: boolean;
  connect: () => Promise<void>;
  fundingDao: any;
  allProposals: Proposal[];
  isStakeholder: boolean;
  isMember: boolean;
  currentBal: string;
  allVotes: string[];
  allInvestedProposal: Proposal[];
  createStakeholder: (amount: string) => Promise<void>;
  provideFunds: (id: string, amount: string) => Promise<void>;
  getProposal: (id: string) => Promise<Proposal>;
  vote: (id: string, vote: boolean) => Promise<void>;
  releaseFunding: (id: string) => Promise<void>;
  createProposal: ({
    title,
    description,
    amount,
    recipient,
    imageId,
  }: {
    title: string;
    description: string;
    amount: string;
    recipient: string;
    imageId: string;
  }) => Promise<void>;
}

const DataContext = createContext<DataContextProps>({
  account: "",
  loading: true,
  connect: async () => {},
  fundingDao: null,
  allProposals: [],
  isStakeholder: false,
  isMember: false,
  currentBal: "",
  allVotes: [],
  allInvestedProposal: [],
  createStakeholder: async (val) => {},
  provideFunds: async (id, amount) => {},
  createProposal: async () => {},
  vote: async () => {},
  releaseFunding: async () => {},
  getProposal: async (val) => {
    return {} as Proposal;
  },
});

export const DataProvider: React.FC = ({ children }) => {
  const data = useProviderData();

  return <DataContext.Provider value={data}>{children}</DataContext.Provider>;
};

export const useData = () => useContext<DataContextProps>(DataContext);

export const useProviderData = () => {
  const [loading, setLoading] = useState(true);
  const [account, setAccount] = useState("");
  const [fundingDao, setFundingDao] = useState<any>();
  const [allProposals, setAllProposals] = useState<Proposal[]>([]);
  const [isStakeholder, setIsStakeholder] = useState(false);
  const [isMember, setIsMember] = useState(false);
  const [currentBal, setCurrentBal] = useState("");
  const [allVotes, setAllVotes] = useState<string[]>([]);
  const [allInvestedProposal, setAllInvestedProposal] = useState<Proposal[]>(
    []
  );

  useEffect(() => {
    connect();
  }, []);

  const connect = async () => {
 
    var allAccounts = ["", "", "", ""];
    setAccount(allAccounts[0]);
    await loadBlockchainData();
  };

  const loadBlockchainData = async () => {
    
  };

  const createStakeholder = async (amount: string) => {
    if (amount === "" || amount === "0") {
      toast.error("Please enter valid amount", {});
    }
    console.log(`fundingDao`, fundingDao);
   // async call goes here 
    loadBlockchainData();
  };

  const createProposal = async ({
    title,
    description,
    amount,
    recipient,
    imageId,
  }: {
    title: string;
    description: string;
    amount: string;
    recipient: string;
    imageId: string;
  }) => {
    if (amount === "" || amount === "0") {
      toast.error("Please enter valid amount", {});
    }
   // async call goes here 
   
    loadBlockchainData();
  };

  const getProposal = async (id: string) => {
 
  };

  const vote = async (id: string, vote: boolean) => {
 
    loadBlockchainData();
  };

  const provideFunds = async (id: string, amount: string) => {
   
    loadBlockchainData();
  };

  const releaseFunding = async (id: string) => {
  
    loadBlockchainData();
  };

  return {
    account,
    fundingDao,
    loading,
    allProposals,
    isStakeholder,
    isMember,
    currentBal,
    allVotes,
    allInvestedProposal,
    connect,
    createStakeholder,
    createProposal,
    getProposal,
    provideFunds,
    releaseFunding,
    vote,
  };
};
